#![no_std]
#![no_main]

//! Periodic acquisition and processing, from section 10.3.5.
//!
//! `sample_current()` reads a measurement every 10 ms with a `Ticker` and
//! queues it in a bounded `Channel`; `process_samples()` drains the queue.
//! Every 64th measurement the processing step deliberately takes 150 ms, far
//! longer than the queue can absorb (8 entries x 10 ms), so the consumer sees
//! gaps in the sequence numbers and reports them.

use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m_semihosting::hprintln;
use embassy_executor::{Executor, InterruptExecutor};
use embassy_stm32::adc::{Adc, SampleTime, Temperature};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_stm32::{bind_interrupts, interrupt, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Instant, Ticker};
use panic_halt as _;
use static_cell::StaticCell;

// Where does sample_current() run? `true` puts it on the interrupt executor,
// so it preempts a long process_current(). `false` puts both tasks on the
// thread-mode executor, which cannot poll the sampler while a calculation
// runs. Change it and compare the log.
const SAMPLER_PREEMPTS: bool = true;

struct Sample {
    sequence: u32,
    raw: u16,
}

static SAMPLES: Channel<CriticalSectionRawMutex, Sample, 8> = Channel::new();

static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();

// The F1 ADC wakes the waiting task from its end-of-conversion interrupt.
bind_interrupts!(struct Irqs {
    ADC1_2 => embassy_stm32::adc::InterruptHandler<peripherals::ADC1>;
});

#[embassy_executor::task]
async fn sample_current(mut adc: Adc<'static, peripherals::ADC1>, mut channel: Temperature) {
    let mut ticker = Ticker::every(Duration::from_millis(10));
    let mut sequence = 0_u32;

    loop {
        ticker.next().await;
        let raw = adc.read(&mut channel, SampleTime::CYCLES239_5).await;
        let sample = Sample { sequence, raw };
        sequence = sequence.wrapping_add(1);

        if SAMPLES.try_send(sample).is_err() {
            // Discard this new sample when the queue is full.
            // Keep sampling instead of waiting for the consumer.
        }
    }
}

#[embassy_executor::task]
async fn process_samples(mut led: Output<'static>) {
    let mut expected = 0_u32;
    let mut received = 0_u32;
    let mut lost = 0_u32;

    loop {
        let sample = SAMPLES.receive().await;
        let dropped = sample.sequence.wrapping_sub(expected);
        expected = sample.sequence.wrapping_add(1);

        if dropped != 0 {
            lost = lost.wrapping_add(dropped);
            hprintln!("gap: {} samples lost before #{}", dropped, sample.sequence);
            led.set_low(); // Active-low LED on: a gap was seen.
        }
        process_current(sample.raw);
        received = received.wrapping_add(1);

        if received % 500 == 0 {
            hprintln!("received {}, lost {}, last raw {}", received, lost, sample.raw);
            led.set_high(); // LED off again.
        }
    }
}

static CALLS: AtomicU32 = AtomicU32::new(0);

/// Stands in for the processing step. Normally about 1 ms; every 64th call
/// it takes 150 ms, like a long filter update.
fn process_current(_raw: u16) {
    let calls = CALLS.fetch_add(1, Ordering::Relaxed);
    let busy_ms = if calls % 64 == 63 { 150 } else { 1 };
    let end = Instant::now() + Duration::from_millis(busy_ms);
    while Instant::now() < end {
        core::hint::spin_loop();
    }
}

// USART2 is reserved for the high-priority executor, not UART traffic.
#[interrupt]
unsafe fn USART2() {
    unsafe { EXECUTOR_HIGH.on_interrupt() }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    hprintln!("current sampling demo ready");

    // The internal temperature sensor stands in for a motor-current sensor.
    let adc = Adc::new(p.ADC1);
    let channel = adc.enable_temperature();
    let led = Output::new(p.PB8, Level::High, Speed::Low);

    interrupt::USART2.set_priority(Priority::P6);
    let high_spawner = EXECUTOR_HIGH.start(interrupt::USART2);

    let executor = EXECUTOR_LOW.init(Executor::new());
    if SAMPLER_PREEMPTS {
        high_spawner.spawn(sample_current(adc, channel).unwrap());
        executor.run(|spawner| {
            spawner.spawn(process_samples(led).unwrap());
        });
    } else {
        executor.run(|spawner| {
            spawner.spawn(sample_current(adc, channel).unwrap());
            spawner.spawn(process_samples(led).unwrap());
        });
    }
}
