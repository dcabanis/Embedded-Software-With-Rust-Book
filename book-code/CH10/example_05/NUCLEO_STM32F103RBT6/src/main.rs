#![no_std]
#![no_main]

use panic_halt as _;
use rtic_monotonics::systick::prelude::*;

systick_monotonic!(Mono, 1_000); // One tick per millisecond.

#[rtic::app(
    device = stm32f1xx_hal::pac,
    dispatchers = [USART1, USART2]
)]
mod app {
    use super::Mono;
    use cortex_m_semihosting::hprintln;
    use rtic_monotonics::systick::prelude::*;
    use stm32f1xx_hal::{
        gpio::{
            gpioa::PA5,
            gpiob::{PB8, PB9},
            gpioc::PC13,
            Edge, ExtiPin, Floating, Input, Output, PushPull,
        },
        prelude::*,
        rcc::Config,
    };

    #[derive(Clone, Copy)]
    enum Phase {
        Green,
        Yellow,
        Red,
    }

    pub struct Controller {
        green: PB8<Output<PushPull>>,
        yellow: PB9<Output<PushPull>>,
        red: PA5<Output<PushPull>>,
        phase: Phase,
        override_active: bool,
        remaining_ms: u32,
    }

    impl Controller {
        fn show(&mut self, phase: Phase) {
            // Switch off the previous output before enabling the next one.
            // LD2 (red, PA5) is active-high; the two external LEDs (green,
            // yellow) are assumed wired the same way (GPIO -> LED -> GND).
            self.green.set_low();
            self.yellow.set_low();
            self.red.set_low();

            match phase {
                Phase::Green => self.green.set_high(),
                Phase::Yellow => self.yellow.set_high(),
                Phase::Red => self.red.set_high(),
            }

            self.phase = phase;
        }

        fn advance(&mut self) {
            let (next, duration_ms) = match self.phase {
                Phase::Red => (Phase::Green, 10_000),
                Phase::Green => (Phase::Yellow, 3_000),
                Phase::Yellow => (Phase::Red, 8_000),
            };

            self.show(next);
            self.remaining_ms = duration_ms;
        }
    }

    #[shared]
    struct Shared {
        controller: Controller,
    }

    #[local]
    struct Local {
        emergency_btn: PC13<Input<Floating>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut flash = ctx.device.FLASH.constrain();
        let rcc = ctx.device.RCC.constrain();
        // stm32f1xx-hal 0.11 takes a `Config` passed to `Rcc::freeze()`;
        // see ../bluepill_STM32F103C8T6/README.md for the 0.10 contrast.
        let mut rcc = rcc.freeze(
            Config::hse(8.MHz()).sysclk(36.MHz()).pclk1(36.MHz()),
            &mut flash.acr,
        );

        // Start SysTick using the actual configured CPU clock frequency.
        Mono::start(ctx.core.SYST, rcc.clocks.sysclk().raw());

        let mut afio = ctx.device.AFIO.constrain(&mut rcc);
        let mut exti = ctx.device.EXTI;
        let mut gpioa = ctx.device.GPIOA.split(&mut rcc);
        let mut gpiob = ctx.device.GPIOB.split(&mut rcc);
        let mut gpioc = ctx.device.GPIOC.split(&mut rcc);

        let mut controller = Controller {
            // Two external LEDs: wire each between the pin and ground
            // through a current-limiting resistor (active-high).
            green: gpiob.pb8.into_push_pull_output(&mut gpiob.crh),
            yellow: gpiob.pb9.into_push_pull_output(&mut gpiob.crh),
            // LD2, the board's own LED: no wiring required.
            red: gpioa.pa5.into_push_pull_output(&mut gpioa.crl),
            phase: Phase::Red,
            override_active: false,
            remaining_ms: 0,
        };
        controller.show(Phase::Red);

        // B1, the onboard USER button, is already pulled up on the Nucleo
        // board and pulls PC13 to ground when pressed, so the pin is left
        // floating here rather than fighting that pull-up internally —
        // matching examples 01-03's NUCLEO ports.
        let mut button = gpioc.pc13.into_floating_input(&mut gpioc.crh);
        button.make_interrupt_source(&mut afio);
        button.trigger_on_edge(&mut exti, Edge::Falling);
        button.clear_interrupt_pending_bit();
        button.enable_interrupt(&mut exti);

        hprintln!("traffic light ready");

        // Request the background task; it starts after init returns.
        traffic_cycle::spawn().unwrap();

        (
            Shared { controller },
            Local {
                emergency_btn: button,
            },
        )
    }

    const CHECK_MS: u32 = 100;

    #[task(priority = 1, shared = [controller])]
    async fn traffic_cycle(mut ctx: traffic_cycle::Context) {
        loop {
            ctx.shared.controller.lock(|controller| {
                // Change color only when the countdown has finished
                // and the emergency hold is inactive.
                if !controller.override_active && controller.remaining_ms == 0 {
                    controller.advance();
                }
            });

            // Wait without holding the controller lock.
            Mono::delay(CHECK_MS.millis()).await;

            ctx.shared.controller.lock(|controller| {
                if !controller.override_active {
                    // Reduce the countdown, stopping at zero.
                    controller.remaining_ms = controller.remaining_ms.saturating_sub(CHECK_MS);
                }
            });
        }
    }

    // Shared by EXTI lines 10-15; only PC13 is enabled here, but checking
    // which pin actually fired keeps the handler correct if another line
    // in that range is ever added — matching examples 01-03's NUCLEO
    // ports. The bluepill port binds its dedicated EXTI3 instead, since
    // its button pin (PB3) doesn't share a vector with anything.
    #[task(
        binds = EXTI15_10,
        priority = 3,
        shared = [controller],
        local = [emergency_btn]
    )]
    fn emergency_override(mut ctx: emergency_override::Context) {
        if !ctx.local.emergency_btn.check_interrupt() {
            return;
        }
        ctx.local.emergency_btn.clear_interrupt_pending_bit();

        // Begin a hold only if no resume_normal task is already active.
        if resume_normal::spawn().is_ok() {
            hprintln!("emergency override engaged");
            ctx.shared.controller.lock(|controller| {
                controller.override_active = true;
                controller.remaining_ms = 0;
                controller.show(Phase::Red);
            });
        }
    }

    #[task(priority = 2, shared = [controller])]
    async fn resume_normal(mut ctx: resume_normal::Context) {
        // Let the emergency red light remain on for fifteen seconds.
        Mono::delay(15_000.millis()).await;

        // Permit the normal task to change the lights again.
        ctx.shared.controller.lock(|controller| {
            controller.override_active = false;
        });
        hprintln!("emergency override released");
    }
}
