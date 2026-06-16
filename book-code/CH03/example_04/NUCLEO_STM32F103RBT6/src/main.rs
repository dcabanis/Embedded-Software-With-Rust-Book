#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f1xx_hal::{
    afio::Remap,
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
    rcc::Config,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // --- RCC / clocks (0.11.0 style) ---------------------------------------
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // freeze() returns an Rcc object in 0.11.0, which contains the Clocks
    rcc = rcc.freeze(
        Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz()),
        &mut flash.acr,
    );

    let clocks = rcc.clocks;

    // --- AFIO / GPIO --------------------------------------------------------
    // In 0.11.0 these need &mut rcc
    let mut afio = dp.AFIO.constrain(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // NUCLEO-F103RB:
    // - Arduino I2C pins are PB8/PB9 (I2C1 remapped)
    // - User LED (LD2) is PA5
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    led.set_high();

    // --- Delay --------------------------------------------------------------
    let mut delay = cp.SYST.delay(&clocks);

    // --- I2C (I2C1 remapped to PB8/PB9) -----------------------------------
    let i2c = BlockingI2c::new::<1>(
        dp.I2C1.remap(&mut afio.mapr),
        (scl, sda),
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        &mut rcc,
        1000, // start_timeout_us
        10,   // start_retries
        1000, // addr_timeout_us
        1000, // data_timeout_us
    );

    // --- Sensor -------------------------------------------------------------
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();

    sensor
        .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz50)
        .unwrap();

    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let a = sensor.acceleration().unwrap();

            if a.x_mg().abs() > 300 {
                led.set_low();
            } else {
                led.set_high();
            }
        }

        delay.delay_ms(50_u16);
    }
}
