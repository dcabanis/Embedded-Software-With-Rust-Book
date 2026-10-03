#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    stm32f1xx_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 2u8; use super ::
    Mono; use cortex_m_semihosting :: hprintln; use rtic_monotonics :: systick
    :: prelude :: * ; use stm32f1xx_hal ::
    {
        gpio ::
        {
            gpioa :: PA5, gpiob :: { PB8, PB9 }, gpioc :: PC13, Edge, ExtiPin,
            Floating, Input, Output, PushPull,
        }, prelude :: * , rcc :: Config,
    }; #[derive(Clone, Copy)] enum Phase { Green, Yellow, Red, } pub struct
    Controller
    {
        green : PB8 < Output < PushPull > > , yellow : PB9 < Output < PushPull
        > > , red : PA5 < Output < PushPull > > , phase : Phase,
        override_active : bool, remaining_ms : u32,
    } impl Controller
    {
        fn show(& mut self, phase : Phase)
        {
            self.green.set_low(); self.yellow.set_low(); self.red.set_low();
            match phase
            {
                Phase :: Green => self.green.set_high(), Phase :: Yellow =>
                self.yellow.set_high(), Phase :: Red => self.red.set_high(),
            } self.phase = phase;
        } fn advance(& mut self)
        {
            let (next, duration_ms) = match self.phase
            {
                Phase :: Red => (Phase :: Green, 10_000), Phase :: Green =>
                (Phase :: Yellow, 3_000), Phase :: Yellow =>
                (Phase :: Red, 8_000),
            }; self.show(next); self.remaining_ms = duration_ms;
        }
    } const CHECK_MS : u32 = 100; #[doc = r" User code end"]
    #[doc = r"Shared resources"] struct Shared { controller : Controller, }
    #[doc = r"Local resources"] struct Local
    { emergency_btn : PC13 < Input < Floating > > , }
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > ,
        #[doc = r" The space used to allocate async executors in bytes."] pub
        executors_size : usize, #[doc = r" Core peripherals"] pub core : rtic
        :: export :: Peripherals, #[doc = r" Device peripherals (PAC)"] pub
        device : stm32f1xx_hal :: pac :: Peripherals,
        #[doc = r" Critical section token for init"] pub cs : rtic :: export
        :: CriticalSection < 'a > ,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn
        new(core : rtic :: export :: Peripherals, executors_size : usize) ->
        Self
        {
            __rtic_internal_init_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, core :
                core, device : stm32f1xx_hal :: pac :: Peripherals :: steal(),
                cs : rtic :: export :: CriticalSection :: new(),
                executors_size,
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_init_Context as
        Context;
    } #[inline(always)] #[allow(non_snake_case)] fn
    init(ctx : init :: Context) -> (Shared, Local)
    {
        let mut flash = ctx.device.FLASH.constrain(); let rcc =
        ctx.device.RCC.constrain(); let mut rcc =
        rcc.freeze(Config :: hse(8.MHz()).sysclk(36.MHz()).pclk1(36.MHz()), &
        mut flash.acr,); Mono ::
        start(ctx.core.SYST, rcc.clocks.sysclk().raw()); let mut afio =
        ctx.device.AFIO.constrain(& mut rcc); let mut exti = ctx.device.EXTI;
        let mut gpioa = ctx.device.GPIOA.split(& mut rcc); let mut gpiob =
        ctx.device.GPIOB.split(& mut rcc); let mut gpioc =
        ctx.device.GPIOC.split(& mut rcc); let mut controller = Controller
        {
            green : gpiob.pb8.into_push_pull_output(& mut gpiob.crh), yellow :
            gpiob.pb9.into_push_pull_output(& mut gpiob.crh), red :
            gpioa.pa5.into_push_pull_output(& mut gpioa.crl), phase : Phase ::
            Red, override_active : false, remaining_ms : 0,
        }; controller.show(Phase :: Red); let mut button =
        gpioc.pc13.into_floating_input(& mut gpioc.crh);
        button.make_interrupt_source(& mut afio);
        button.trigger_on_edge(& mut exti, Edge :: Falling);
        button.clear_interrupt_pending_bit();
        button.enable_interrupt(& mut exti); hprintln!
        ("traffic light ready"); traffic_cycle :: spawn().unwrap();
        (Shared { controller }, Local { emergency_btn : button, },)
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn EXTI15_10()
    {
        const PRIORITY : u8 = 3u8; rtic :: export ::
        run(PRIORITY, ||
        { emergency_override(emergency_override :: Context :: new()) });
    } impl < 'a > __rtic_internal_emergency_overrideLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_emergency_overrideLocalResources
            {
                emergency_btn : & mut *
                (& mut *
                __rtic_internal_local_resource_emergency_btn.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_emergency_overrideSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_emergency_overrideSharedResources
            {
                controller : shared_resources ::
                controller_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `emergency_override` has access to"] pub struct
    __rtic_internal_emergency_overrideLocalResources < 'a >
    {
        #[allow(missing_docs)] pub emergency_btn : & 'a mut PC13 < Input <
        Floating > > , #[doc(hidden)] pub __rtic_internal_marker : :: core ::
        marker :: PhantomData < & 'a () > ,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `emergency_override` has access to"] pub struct
    __rtic_internal_emergency_overrideSharedResources < 'a >
    {
        #[allow(missing_docs)] pub controller : shared_resources ::
        controller_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_emergency_override_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Local Resources this task has access to"] pub
        local : emergency_override :: LocalResources < 'a > ,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        emergency_override :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_emergency_override_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_emergency_override_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                emergency_override :: LocalResources :: new(), shared :
                emergency_override :: SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod
    emergency_override
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_emergency_overrideLocalResources as LocalResources;
        #[doc(inline)] pub use super ::
        __rtic_internal_emergency_overrideSharedResources as SharedResources;
        #[doc(inline)] pub use super ::
        __rtic_internal_emergency_override_Context as Context;
    } #[allow(non_snake_case)] fn
    emergency_override(mut ctx : emergency_override :: Context)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; if !
        ctx.local.emergency_btn.check_interrupt() { return; }
        ctx.local.emergency_btn.clear_interrupt_pending_bit(); if
        resume_normal :: spawn().is_ok()
        {
            hprintln! ("emergency override engaged");
            ctx.shared.controller.lock(| controller |
            {
                controller.override_active = true; controller.remaining_ms =
                0; controller.show(Phase :: Red);
            });
        }
    } impl < 'a > __rtic_internal_traffic_cycleSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_traffic_cycleSharedResources
            {
                controller : shared_resources ::
                controller_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_resume_normalSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_resume_normalSharedResources
            {
                controller : shared_resources ::
                controller_that_needs_to_be_locked :: new(),
                __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `traffic_cycle` has access to"] pub struct
    __rtic_internal_traffic_cycleSharedResources < 'a >
    {
        #[allow(missing_docs)] pub controller : shared_resources ::
        controller_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_traffic_cycle_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Shared Resources this task has access to"] pub
        shared : traffic_cycle :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_traffic_cycle_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_traffic_cycle_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                traffic_cycle :: SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_traffic_cycle_spawn() -> :: core ::
    result :: Result < (), () >
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(traffic_cycle, &
            __rtic_internal_traffic_cycle_EXEC); if exec.try_allocate()
            {
                exec.spawn(traffic_cycle(unsafe
                { traffic_cycle :: Context :: new() })); rtic :: export ::
                pend(stm32f1xx_hal :: pac :: interrupt :: USART1); Ok(())
            } else { Err(()) }
        }
    } #[doc = r" Gives waker to the task"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_traffic_cycle_waker() -> :: core ::
    task :: Waker
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(traffic_cycle, &
            __rtic_internal_traffic_cycle_EXEC);
            exec.waker(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(traffic_cycle, &
                __rtic_internal_traffic_cycle_EXEC); exec.set_pending(); rtic
                :: export ::
                pend(stm32f1xx_hal :: pac :: interrupt :: USART1);
            })
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod traffic_cycle
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_traffic_cycleSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_traffic_cycle_Context
        as Context; #[doc(inline)] pub use super ::
        __rtic_internal_traffic_cycle_spawn as spawn; #[doc(inline)] pub use
        super :: __rtic_internal_traffic_cycle_waker as waker;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `resume_normal` has access to"] pub struct
    __rtic_internal_resume_normalSharedResources < 'a >
    {
        #[allow(missing_docs)] pub controller : shared_resources ::
        controller_that_needs_to_be_locked < 'a > , #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_resume_normal_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () > , #[doc = r" Shared Resources this task has access to"] pub
        shared : resume_normal :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_resume_normal_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_resume_normal_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                resume_normal :: SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_resume_normal_spawn() -> :: core ::
    result :: Result < (), () >
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(resume_normal, &
            __rtic_internal_resume_normal_EXEC); if exec.try_allocate()
            {
                exec.spawn(resume_normal(unsafe
                { resume_normal :: Context :: new() })); rtic :: export ::
                pend(stm32f1xx_hal :: pac :: interrupt :: USART2); Ok(())
            } else { Err(()) }
        }
    } #[doc = r" Gives waker to the task"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_resume_normal_waker() -> :: core ::
    task :: Waker
    {
        unsafe
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(resume_normal, &
            __rtic_internal_resume_normal_EXEC);
            exec.waker(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(resume_normal, &
                __rtic_internal_resume_normal_EXEC); exec.set_pending(); rtic
                :: export ::
                pend(stm32f1xx_hal :: pac :: interrupt :: USART2);
            })
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod resume_normal
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_resume_normalSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_resume_normal_Context
        as Context; #[doc(inline)] pub use super ::
        __rtic_internal_resume_normal_spawn as spawn; #[doc(inline)] pub use
        super :: __rtic_internal_resume_normal_waker as waker;
    } #[allow(non_snake_case)] async fn traffic_cycle < 'a >
    (mut ctx : traffic_cycle :: Context < 'a >)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; loop
        {
            ctx.shared.controller.lock(| controller |
            {
                if ! controller.override_active && controller.remaining_ms ==
                0 { controller.advance(); }
            }); Mono :: delay(CHECK_MS.millis()).await;
            ctx.shared.controller.lock(| controller |
            {
                if ! controller.override_active
                {
                    controller.remaining_ms =
                    controller.remaining_ms.saturating_sub(CHECK_MS);
                }
            });
        }
    } #[allow(non_snake_case)] async fn resume_normal < 'a >
    (mut ctx : resume_normal :: Context < 'a >)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; Mono ::
        delay(15_000.millis()).await;
        ctx.shared.controller.lock(| controller |
        { controller.override_active = false; }); hprintln!
        ("emergency override released");
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_controller : rtic :: RacyCell < core ::
    mem :: MaybeUninit < Controller >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: controller_that_needs_to_be_locked < 'a >
    {
        type T = Controller; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut Controller) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 3u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_controller.get_mut() as *
                mut _, CEILING, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS, f,)
            }
        }
    } mod shared_resources
    {
        #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        controller_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () > , }
        impl < 'a > controller_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                controller_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_local_resource_emergency_btn : rtic :: RacyCell < core ::
    mem :: MaybeUninit < PC13 < Input < Floating > > >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_upper_case_globals)] static __rtic_internal_traffic_cycle_EXEC
    : rtic :: export :: executor :: AsyncTaskExecutorPtr = rtic :: export ::
    executor :: AsyncTaskExecutorPtr :: new();
    #[allow(non_upper_case_globals)] static __rtic_internal_resume_normal_EXEC
    : rtic :: export :: executor :: AsyncTaskExecutorPtr = rtic :: export ::
    executor :: AsyncTaskExecutorPtr :: new(); #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 1"]
    #[no_mangle] unsafe fn USART1()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8; rtic :: export ::
        run(PRIORITY, ||
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(traffic_cycle, &
            __rtic_internal_traffic_cycle_EXEC);
            exec.poll(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(traffic_cycle, &
                __rtic_internal_traffic_cycle_EXEC); exec.set_pending(); rtic
                :: export ::
                pend(stm32f1xx_hal :: pac :: interrupt :: USART1);
            });
        });
    } #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 2"]
    #[no_mangle] unsafe fn USART2()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 2u8; rtic :: export ::
        run(PRIORITY, ||
        {
            let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
            from_ptr_1_args(resume_normal, &
            __rtic_internal_resume_normal_EXEC);
            exec.poll(||
            {
                let exec = rtic :: export :: executor :: AsyncTaskExecutor ::
                from_ptr_1_args(resume_normal, &
                __rtic_internal_resume_normal_EXEC); exec.set_pending(); rtic
                :: export ::
                pend(stm32f1xx_hal :: pac :: interrupt :: USART2);
            });
        });
    } #[doc(hidden)] #[no_mangle] unsafe extern "C" fn main() -> !
    {
        rtic :: export :: assert_send :: < Controller > (); rtic :: export ::
        interrupt :: disable(); let mut core : rtic :: export :: Peripherals =
        rtic :: export :: Peripherals :: steal().into(); let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: USART1; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: USART2; const _ : () = if
        (1 << stm32f1xx_hal :: pac :: NVIC_PRIO_BITS) < 1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'USART1' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: USART1, rtic :: export ::
        cortex_logical2hw(1u8, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: USART1); const _ : () = if
        (1 << stm32f1xx_hal :: pac :: NVIC_PRIO_BITS) < 2u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'USART2' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: USART2, rtic :: export ::
        cortex_logical2hw(2u8, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: USART2); const _ : () = if
        (1 << stm32f1xx_hal :: pac :: NVIC_PRIO_BITS) < 3u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'EXTI15_10' is more than supported by hardware");
        };
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI15_10, rtic :: export ::
        cortex_logical2hw(3u8, stm32f1xx_hal :: pac :: NVIC_PRIO_BITS),); rtic
        :: export :: NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: EXTI15_10); #[inline(never)] fn __rtic_init_resources
        < F > (f : F) where F : FnOnce() { f(); } let mut executors_size = 0;
        let executor = :: core :: mem :: ManuallyDrop ::
        new(rtic :: export :: executor :: AsyncTaskExecutor ::
        new_1_args(traffic_cycle)); executors_size += :: core :: mem ::
        size_of_val(& executor);
        __rtic_internal_traffic_cycle_EXEC.set_in_main(& executor); let
        executor = :: core :: mem :: ManuallyDrop ::
        new(rtic :: export :: executor :: AsyncTaskExecutor ::
        new_1_args(resume_normal)); executors_size += :: core :: mem ::
        size_of_val(& executor);
        __rtic_internal_resume_normal_EXEC.set_in_main(& executor); extern "C"
        { pub static _stack_start : u32; pub static __ebss : u32; } let
        stack_start = & _stack_start as * const _ as u32; let ebss = & __ebss
        as * const _ as u32; if stack_start > ebss
        {
            if rtic :: export :: msp :: read() <= ebss
            { panic! ("Stack overflow after allocating executors"); }
        }
        __rtic_init_resources(||
        {
            let (shared_resources, local_resources) =
            init(init :: Context :: new(core.into(), executors_size));
            __rtic_internal_shared_resource_controller.get_mut().write(core ::
            mem :: MaybeUninit :: new(shared_resources.controller));
            __rtic_internal_local_resource_emergency_btn.get_mut().write(core
            :: mem :: MaybeUninit :: new(local_resources.emergency_btn)); rtic
            :: export :: interrupt :: enable();
        }); loop {}
    }
}