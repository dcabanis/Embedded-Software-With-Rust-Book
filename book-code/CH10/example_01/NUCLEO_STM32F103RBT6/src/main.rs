#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f1xx_hal::{
    gpio::{
        gpioa::{PA0, PA5},
        gpioc::PC13,
        Edge, ExtiPin, Floating, Input, Output, PullDown, PushPull,
    },
    pac::{self, interrupt},
    prelude::*,
};

use app::{Event, Fan, FanState};

static FAN: Mutex<RefCell<Fan>> = Mutex::new(RefCell::new(Fan::new()));
static BUTTON_PIN: Mutex<RefCell<Option<PC13<Input<Floating>>>>> =
    Mutex::new(RefCell::new(None));
static STALL_PIN: Mutex<RefCell<Option<PA0<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));

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
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // LD2 on the NUCLEO-F103RB: active-high, stands in for the board's
    // fault indicator since no motor driver is attached.
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    led.set_low();

    // B1, the onboard USER button, is already pulled up on the Nucleo
    // board and pulls PC13 to ground when pressed, so the pin is left
    // floating here rather than fighting that pull-up internally.
    let mut button = gpioc.pc13.into_floating_input(&mut gpioc.crh);
    button.make_interrupt_source(&mut afio);
    button.trigger_on_edge(&mut exti, Edge::Falling);
    button.clear_interrupt_pending_bit();
    button.enable_interrupt(&mut exti);

    // Stands in for a tachometer or other stall sensor, which section
    // 10.2.2 leaves outside the example. There is no onboard second button
    // on this board, so this is an external circuit: a momentary button
    // wired between PA0 and 3.3 V, with the internal pull-down giving the
    // released input a defined low level and a rising edge marking the
    // press.
    let mut stall = gpioa.pa0.into_pull_down_input(&mut gpioa.crl);
    stall.make_interrupt_source(&mut afio);
    stall.trigger_on_edge(&mut exti, Edge::Rising);
    stall.clear_interrupt_pending_bit();
    stall.enable_interrupt(&mut exti);

    cortex_m::interrupt::free(|cs| {
        BUTTON_PIN.borrow(cs).replace(Some(button));
        STALL_PIN.borrow(cs).replace(Some(stall));
    });
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI15_10);
        pac::NVIC::unmask(pac::Interrupt::EXTI0);
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

fn apply_state(state: FanState, led: &mut PA5<Output<PushPull>>) {
    match state {
        FanState::Off => set_outputs(0, false, led),
        FanState::Low => set_outputs(1, false, led),
        FanState::Medium => set_outputs(2, false, led),
        FanState::High => set_outputs(3, false, led),
        FanState::Stalled => set_outputs(0, true, led),
    }
}

fn set_outputs(speed: u8, fault: bool, led: &mut PA5<Output<PushPull>>) {
    set_fan_speed(speed);
    set_fault_indicator(fault, led);
}

fn set_fan_speed(speed: u8) {
    hprintln!("fan speed = {}", speed);
}

fn set_fault_indicator(active: bool, led: &mut PA5<Output<PushPull>>) {
    if active {
        led.set_high();
    } else {
        led.set_low();
    }
}

// Shared by EXTI lines 10-15; only PC13 is enabled here, but checking
// which pin actually fired keeps the handler correct if another line
// in that range is ever added.
#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        if let Some(pin) = BUTTON_PIN.borrow(cs).borrow_mut().as_mut() {
            if pin.check_interrupt() {
                pin.clear_interrupt_pending_bit();
                FAN.borrow(cs).borrow_mut().handle(Event::ButtonPress);
            }
        }
    });
}

// EXTI0 is a dedicated vector (PA0 is its only possible source), so unlike
// EXTI15_10 above, no check_interrupt() disambiguation is needed here.
#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(pin) = STALL_PIN.borrow(cs).borrow_mut().as_mut() {
            pin.clear_interrupt_pending_bit();
        }
        FAN.borrow(cs).borrow_mut().handle(Event::StallDetected);
    });
}
