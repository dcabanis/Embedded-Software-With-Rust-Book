
#![no_main]
#![no_std]

use core::convert::TryInto;

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f3xx_hal::{
    delay::Delay,
    i2c::I2c,
    pac,
    prelude::*,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

#[entry]
fn main() -> ! {
    // Peripheral singletons: `dp` for device peripherals, `cp` for Cortex-M core peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Clock tree configuration
    // - External 8 MHz crystal (HSE) is used as the source
    // - SYSCLK is raised to 72 MHz
    // - APB1 is kept at 36 MHz
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz())
        .freeze(&mut flash.acr);

    // Split GPIO peripherals so individual pins can be configured safely
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // I2C1 pins on this board:
    // - PB6: SCL
    // - PB7: SDA
    // Open-drain + pull-ups are required for I2C signaling
    let mut scl =
        gpiob
            .pb6
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    scl.internal_pull_up(&mut gpiob.pupdr, true);
    sda.internal_pull_up(&mut gpiob.pupdr, true);

    // User LED (PE13): output indicator driven by acceleration threshold logic
    let mut led = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let _ = led.set_low();

    // SysTick-based blocking delay provider used for simple pacing in the main loop
    let mut delay = Delay::new(cp.SYST, clocks);

    // Concrete STM32 I2C peripheral instance configured at 400 kHz.
    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        400.kHz().try_into().unwrap(),
        clocks,
        &mut rcc.apb1,
    );

    // High-level flow:
    // 1) `stm32f3xx-hal` configures board-specific pins/peripherals and exposes an I2C type
    //    implementing embedded-hal traits
    // 2) `lsm303agr` only depends on those generic embedded-hal traits, so the same sensor
    //    control logic can run on other MCUs/boards with a different HAL
    // 3) Sensor driver initialization configures the device over I2C once at startup
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_mode(AccelMode::Normal).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    // The application loop is device-agnostic business logic: read accel data and drive LED
    loop {
        // Poll status first to avoid reading stale acceleration samples
        if sensor.accel_status().unwrap().xyz_new_data {
            let a = sensor.accel_data().unwrap();

            // Simple threshold on X-axis magnitude.
            // If motion/tilt exceeds the threshold, turn LED on; otherwise off
            if a.x.abs() > 300 {
                let _ = led.set_high();
            } else {
                let _ = led.set_low();
            }
        }

        // 50 ms pacing reduces bus traffic and CPU usage in this polling-based example
        delay.delay_ms(50_u16);
    }
}
