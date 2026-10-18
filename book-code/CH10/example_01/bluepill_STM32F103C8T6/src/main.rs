#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal::{
    gpio::{
        gpiob::{PB3, PB4, PB8},
        Edge, ExtiPin, Input, Output, PullUp, PushPull,
    },
    pac::{self, interrupt},
    prelude::*,
};

use app::{Event, Fan, FanState};

static FAN: Mutex<RefCell<Fan>> = Mutex::new(RefCell::new(Fan::new()));
static BUTTON_PIN: Mutex<RefCell<Option<PB3<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
static STALL_PIN: Mutex<RefCell<Option<PB4<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // STM32F1 gates the debug clock domain during Sleep mode by default
    // (DBGMCU_CR.DBG_SLEEP = 0), so a probe attached over SWD can lose
    // contact with the core the moment `wfi()` runs below, breaking the
    // SWD connection for any later reflash/reconnect until the board is
    // reset. Setting this bit keeps the debug clock alive through Sleep.
    dp.DBGMCU.cr().modify(|_, w| w.dbg_sleep().set_bit());

    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc);
    let mut exti = dp.EXTI;
    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);

    // PB3 and PB4 default to JTAG signals (JTDO and JTRST) on this chip,
    // so they must be released before they behave as plain GPIO. This also
    // frees PA15 (JTDI) as a side effect of the same call, even though this
    // example does not use it. SWD debugging (what probe-rs uses here)
    // keeps working afterward — only the extra JTAG-only pins go away.
    let (_pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // This board's LEDs are active-low: LOW turns them on, HIGH turns
    // them off. PB8 stands in for the board's fault indicator since no
    // motor driver is attached.
    let mut led = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
    led.set_high();

    // Both buttons read active-low with the internal pull-up enabled
    // (idle high, pressed low).
    // PB3 and PB4 each have their own dedicated EXTI vector (EXTI3,
    // EXTI4), unlike PC13 on the NUCLEO port, which shares EXTI15_10
    // with seven other pins — so no interrupt-source disambiguation is
    // needed in either handler below.
    let mut button = pb3.into_pull_up_input(&mut gpiob.crl);
    button.make_interrupt_source(&mut afio);
    button.trigger_on_edge(&mut exti, Edge::Falling);
    button.clear_interrupt_pending_bit();
    button.enable_interrupt(&mut exti);

    // Stands in for a tachometer or other stall sensor, which chapter
    // 10.2.2 leaves outside the example; pressing this button submits
    // Event::StallDetected the same way a real sensor handler would.
    let mut stall = pb4.into_pull_up_input(&mut gpiob.crl);
    stall.make_interrupt_source(&mut afio);
    stall.trigger_on_edge(&mut exti, Edge::Falling);
    stall.clear_interrupt_pending_bit();
    stall.enable_interrupt(&mut exti);

    cortex_m::interrupt::free(|cs| {
        BUTTON_PIN.borrow(cs).replace(Some(button));
        STALL_PIN.borrow(cs).replace(Some(stall));
    });
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI3);
        pac::NVIC::unmask(pac::Interrupt::EXTI4);
    }

    hprintln!("fan controller ready");

    let mut last_state = None;

    loop {
        cortex_m::interrupt::free(|cs| {
            // Read and apply one consistent state while IRQs are masked.
            let state = FAN.borrow(cs).borrow().state();
            if last_state != Some(state) {
                apply_state(state, &mut led);
                last_state = Some(state);
            }

            // Keep the sleep boundary inside the same critical section.
            cortex_m::asm::dsb();
            cortex_m::asm::wfi();
        });
    }
}

fn apply_state(state: FanState, led: &mut PB8<Output<PushPull>>) {
    match state {
        FanState::Off => set_outputs(0, false, led),
        FanState::Low => set_outputs(1, false, led),
        FanState::Medium => set_outputs(2, false, led),
        FanState::High => set_outputs(3, false, led),
        FanState::Stalled => set_outputs(0, true, led),
    }
}

fn set_outputs(speed: u8, fault: bool, led: &mut PB8<Output<PushPull>>) {
    set_fan_speed(speed);
    set_fault_indicator(fault, led);
}

fn set_fan_speed(speed: u8) {
    hprintln!("fan speed = {}", speed);
}

fn set_fault_indicator(active: bool, led: &mut PB8<Output<PushPull>>) {
    // Active-low LED: LOW = on, HIGH = off.
    if active {
        led.set_low();
    } else {
        led.set_high();
    }
}

#[interrupt]
fn EXTI3() {
    cortex_m::interrupt::free(|cs| {
        if let Some(pin) = BUTTON_PIN.borrow(cs).borrow_mut().as_mut() {
            pin.clear_interrupt_pending_bit();
        }
        FAN.borrow(cs).borrow_mut().handle(Event::ButtonPress);
    });
}

#[interrupt]
fn EXTI4() {
    cortex_m::interrupt::free(|cs| {
        if let Some(pin) = STALL_PIN.borrow(cs).borrow_mut().as_mut() {
            pin.clear_interrupt_pending_bit();
        }
        FAN.borrow(cs).borrow_mut().handle(Event::StallDetected);
    });
}
