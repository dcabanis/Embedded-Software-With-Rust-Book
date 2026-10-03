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
            gpioa::PA9,
            gpiob::{PB3, PB8, PB9},
            Edge, ExtiPin, Input, Output, PullUp, PushPull,
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
        red: PA9<Output<PushPull>>,
        phase: Phase,
        override_active: bool,
        remaining_ms: u32,
    }

    impl Controller {
        fn show(&mut self, phase: Phase) {
            // Switch off the previous output before enabling the next one.
            // These LEDs are active-low: HIGH = off, LOW = on.
            self.green.set_high();
            self.yellow.set_high();
            self.red.set_high();

            match phase {
                Phase::Green => self.green.set_low(),
                Phase::Yellow => self.yellow.set_low(),
                Phase::Red => self.red.set_low(),
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
        emergency_btn: PB3<Input<PullUp>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let mut flash = ctx.device.FLASH.constrain();
        let rcc = ctx.device.RCC.constrain();
        // stm32f1xx-hal 0.11 replaced 0.10's `rcc.cfgr` builder (used in
        // the text) with a `Config` passed directly to `Rcc::freeze()`;
        // the resulting `Rcc`'s `clocks` field replaces the separate
        // `Clocks` value 0.10's `freeze()` used to return.
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

        // PB3 defaults to a JTAG signal (JTDO) on this chip, so it must be
        // released before it behaves as plain GPIO. This also frees PA15
        // and PB4 (JTDI and JTRST) as a side effect of the same call, even
        // though this example does not use them. SWD debugging keeps
        // working afterward.
        let (_pa15, pb3, _pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        let mut controller = Controller {
            green: gpiob.pb8.into_push_pull_output(&mut gpiob.crh),
            yellow: gpiob.pb9.into_push_pull_output(&mut gpiob.crh),
            red: gpioa.pa9.into_push_pull_output(&mut gpioa.crh),
            phase: Phase::Red,
            override_active: false,
            remaining_ms: 0,
        };
        controller.show(Phase::Red);

        // This board's buttons read active-low with an internal pull-up
        // (idle high, pressed low), confirmed on this board in examples
        // 01-03 — the opposite polarity from the pull-down/rising-edge
        // circuit section 8.4.2 describes, so we trigger on the falling
        // edge instead.
        let mut button = pb3.into_pull_up_input(&mut gpiob.crl);
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

    #[task(
        binds = EXTI3,
        priority = 3,
        shared = [controller],
        local = [emergency_btn]
    )]
    fn emergency_override(mut ctx: emergency_override::Context) {
        // Acknowledge the button interrupt.
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
