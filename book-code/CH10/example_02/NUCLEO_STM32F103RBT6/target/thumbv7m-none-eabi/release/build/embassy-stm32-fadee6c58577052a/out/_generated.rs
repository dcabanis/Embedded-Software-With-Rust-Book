embassy_hal_internal::peripherals_definition!(
    PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA11, PA12, PA13, PA14, PA15, PB0, PB1,
    PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11, PB12, PB13, PB14, PB15, PC0, PC1, PC2, PC3,
    PC4, PC5, PC6, PC7, PC8, PC9, PC10, PC11, PC12, PC13, PC14, PC15, PD0, PD1, PD2, ADC1, ADC2,
    AFIO, BKP, CAN, CRC, DBGMCU, DMA1, FLASH, I2C1, I2C2, IWDG, PWR, MCO, RCC, RTC, SPI1, SPI2,
    TIM1, TIM2, TIM3, TIM4, UID, USART1, USART2, USART3, USB, USBRAM, WWDG, EXTI0, EXTI1, EXTI2,
    EXTI3, EXTI4, EXTI5, EXTI6, EXTI7, EXTI8, EXTI9, EXTI10, EXTI11, EXTI12, EXTI13, EXTI14,
    EXTI15, DMA1_CH1, DMA1_CH2, DMA1_CH3, DMA1_CH4, DMA1_CH5, DMA1_CH6, DMA1_CH7
);
embassy_hal_internal::peripherals_struct!(
    PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA11, PA12, PA13, PA14, PA15, PB0, PB1,
    PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11, PB12, PB13, PB14, PB15, PC0, PC1, PC2, PC3,
    PC4, PC5, PC6, PC7, PC8, PC9, PC10, PC11, PC12, PC13, PC14, PC15, PD0, PD1, PD2, ADC1, ADC2,
    AFIO, BKP, CAN, CRC, DBGMCU, DMA1, FLASH, I2C1, I2C2, IWDG, PWR, MCO, RCC, RTC, SPI1, SPI2,
    TIM1, TIM3, TIM4, UID, USART1, USART2, USART3, USB, USBRAM, WWDG, EXTI0, EXTI1, EXTI2, EXTI3,
    EXTI4, EXTI5, EXTI6, EXTI7, EXTI8, EXTI9, EXTI10, EXTI11, EXTI12, EXTI13, EXTI14, EXTI15,
    DMA1_CH1, DMA1_CH2, DMA1_CH3, DMA1_CH4, DMA1_CH5, DMA1_CH6, DMA1_CH7
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD,
    TAMPER,
    RTC,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_CHANNEL1,
    DMA1_CHANNEL2,
    DMA1_CHANNEL3,
    DMA1_CHANNEL4,
    DMA1_CHANNEL5,
    DMA1_CHANNEL6,
    DMA1_CHANNEL7,
    ADC1_2,
    USB_HP_CAN1_TX,
    USB_LP_CAN1_RX0,
    CAN1_RX1,
    CAN1_SCE,
    EXTI9_5,
    TIM1_BRK,
    TIM1_UP,
    TIM1_TRG_COM,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    USBWAKEUP,
);
pub const MAX_ERASE_SIZE: usize = 1024u32 as usize;
pub mod flash_regions {
    pub const BANK1_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        base: 134217728u32,
        size: 131072u32,
        erase_size: 1024u32,
        write_size: 4u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region: Bank1Region<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(p: embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>) -> Self {
            Self {
                bank1_region: Bank1Region(
                    &BANK1_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 1usize] = [&BANK1_REGION];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 9u8)),
            (6u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 10u8)),
            (6u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::AFIO {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "AFIO" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "AFIO" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 0u8)),
            (6u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::AFIO {}
impl crate::rcc::SealedRccPeripheral for peripherals::BKP {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "BKP" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "BKP" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 27u8)),
            (7u8, 27u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::BKP {}
impl crate::rcc::SealedRccPeripheral for peripherals::CAN {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CAN" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 25u8)),
            (7u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CAN {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (5u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (5u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FLASH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FLASH" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (5u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FLASH {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 21u8)),
            (7u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 22u8)),
            (7u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::PWR {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "PWR" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 28u8)),
            (7u8, 28u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::PWR {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 12u8)),
            (6u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 14u8)),
            (7u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 11u8)),
            (6u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 0u8)),
            (7u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 1u8)),
            (7u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 2u8)),
            (7u8, 2u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((3u8, 14u8)),
            (6u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 17u8)),
            (7u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 18u8)),
            (7u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . usb . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB" , "usb")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 23u8)),
            (7u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((4u8, 11u8)),
            (7u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG {}
pub(crate) static mut REFCOUNTS: [u8; 0usize] = [];
pub mod mux {
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {}
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {}
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub hclk1: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pclk2: crate::time::MaybeHertz,
    pub pclk2_tim: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
    pub usb: crate::time::MaybeHertz,
}
pub unsafe fn init_dma() {}
pub unsafe fn init_bdma() {
    crate::pac::RCC.ahbenr().modify(|w| w.set_dma1en(true));
}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.apb2enr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.apb2enr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.apb2enr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.apb2enr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.apb2enr().modify(|w| w.set_gpioeen(true));
}
impl_adc_pin!(ADC1, PA0, 0u8);
impl_adc_pin!(ADC1, PA1, 1u8);
impl_adc_pin!(ADC1, PA2, 2u8);
impl_adc_pin!(ADC1, PA3, 3u8);
impl_adc_pin!(ADC1, PA4, 4u8);
impl_adc_pin!(ADC1, PA5, 5u8);
impl_adc_pin!(ADC1, PA6, 6u8);
impl_adc_pin!(ADC1, PA7, 7u8);
impl_adc_pin!(ADC1, PB0, 8u8);
impl_adc_pin!(ADC1, PB1, 9u8);
impl_adc_pin!(ADC1, PC0, 10u8);
impl_adc_pin!(ADC1, PC1, 11u8);
impl_adc_pin!(ADC1, PC2, 12u8);
impl_adc_pin!(ADC1, PC3, 13u8);
impl_adc_pin!(ADC1, PC4, 14u8);
impl_adc_pin!(ADC1, PC5, 15u8);
impl_adc_pin!(ADC2, PA0, 0u8);
impl_adc_pin!(ADC2, PA1, 1u8);
impl_adc_pin!(ADC2, PA2, 2u8);
impl_adc_pin!(ADC2, PA3, 3u8);
impl_adc_pin!(ADC2, PA4, 4u8);
impl_adc_pin!(ADC2, PA5, 5u8);
impl_adc_pin!(ADC2, PA6, 6u8);
impl_adc_pin!(ADC2, PA7, 7u8);
impl_adc_pin!(ADC2, PB0, 8u8);
impl_adc_pin!(ADC2, PB1, 9u8);
impl_adc_pin!(ADC2, PC0, 10u8);
impl_adc_pin!(ADC2, PC1, 11u8);
impl_adc_pin!(ADC2, PC2, 12u8);
impl_adc_pin!(ADC2, PC3, 13u8);
impl_adc_pin!(ADC2, PC4, 14u8);
impl_adc_pin!(ADC2, PC5, 15u8);
pin_trait_impl!(crate::can::RxPin, CAN, PA11, 0u8);
pin_trait_impl!(crate::can::TxPin, CAN, PA12, 0u8);
pin_trait_impl!(crate::can::RxPin, CAN, PB8, 0u8);
pin_trait_impl!(crate::can::TxPin, CAN, PB9, 0u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB6, 0u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB7, 0u8);
pin_trait_impl!(crate::i2c::SclPin, I2C1, PB8, 0u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C1, PB9, 0u8);
pin_trait_impl!(crate::i2c::SclPin, I2C2, PB10, 0u8);
pin_trait_impl!(crate::i2c::SdaPin, I2C2, PB11, 0u8);
pin_trait_impl!(crate::rcc::McoPin, MCO, PA8, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA15, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI1, PA4, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PA5, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PA6, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PA7, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI1, PB3, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI1, PB4, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI1, PB5, 0u8);
pin_trait_impl!(crate::spi::CsPin, SPI2, PB12, 0u8);
pin_trait_impl!(crate::spi::SckPin, SPI2, PB13, 0u8);
pin_trait_impl!(crate::spi::MisoPin, SPI2, PB14, 0u8);
pin_trait_impl!(crate::spi::MosiPin, SPI2, PB15, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM1, PA10, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM1, PA11, 0u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM1, PA12, 0u8);
pin_trait_impl!(crate::timer::BreakInputPin<BkIn1>, TIM1, PA6, 0u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM1, PA7, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM1, PA8, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM1, PA9, 0u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch2>, TIM1, PB0, 0u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch3>, TIM1, PB1, 0u8);
pin_trait_impl!(crate::timer::BreakInputPin<BkIn1>, TIM1, PB12, 0u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch1>, TIM1, PB13, 0u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch2>, TIM1, PB14, 0u8);
pin_trait_impl!(crate::timer::TimerComplementaryPin<Ch3>, TIM1, PB15, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM2, PA0, 0u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA0, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM2, PA1, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM2, PA15, 0u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM2, PA15, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM2, PA2, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM2, PA3, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM2, PB10, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM2, PB11, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM2, PB3, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM3, PA6, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM3, PA7, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM3, PB0, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM3, PB1, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM3, PB4, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM3, PB5, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM3, PC6, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM3, PC7, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM3, PC8, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM3, PC9, 0u8);
pin_trait_impl!(crate::timer::ExternalTriggerPin, TIM3, PD2, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch1>, TIM4, PB6, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch2>, TIM4, PB7, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch3>, TIM4, PB8, 0u8);
pin_trait_impl!(crate::timer::TimerPin<Ch4>, TIM4, PB9, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PA10, 0u8);
pin_trait_impl!(crate::usart::CtsPin, USART1, PA11, 0u8);
pin_trait_impl!(crate::usart::RtsPin, USART1, PA12, 0u8);
pin_trait_impl!(crate::usart::CkPin, USART1, PA8, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PA9, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART1, PB6, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART1, PB7, 0u8);
pin_trait_impl!(crate::usart::CtsPin, USART2, PA0, 0u8);
pin_trait_impl!(crate::usart::RtsPin, USART2, PA1, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART2, PA2, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART2, PA3, 0u8);
pin_trait_impl!(crate::usart::CkPin, USART2, PA4, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PB10, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PB11, 0u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PB12, 0u8);
pin_trait_impl!(crate::usart::CtsPin, USART3, PB13, 0u8);
pin_trait_impl!(crate::usart::RtsPin, USART3, PB14, 0u8);
pin_trait_impl!(crate::usart::TxPin, USART3, PC10, 0u8);
pin_trait_impl!(crate::usart::RxPin, USART3, PC11, 0u8);
pin_trait_impl!(crate::usart::CkPin, USART3, PC12, 0u8);
pin_trait_impl!(crate::usb::DmPin, USB, PA11, 0u8);
pin_trait_impl!(crate::usb::DpPin, USB, PA12, 0u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH1, (), {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH6, (), {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH7, (), {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH4, (), {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH5, (), {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH2, (), {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH3, (), {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH4, (), {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH5, (), {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH2, (), {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH3, (), {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH4, (), {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH5, (), {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH6, (), {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH1, (), {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH2, (), {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH5, (), {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH7, (), {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH7, (), {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH2, (), {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH3, (), {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH3, (), {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH6, (), {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH1, (), {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH4, (), {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH5, (), {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH7, (), {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH4, (), {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH5, (), {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH6, (), {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH7, (), {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH2, (), {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH3, (), {});
impl core::ops::Div<crate::pac::rcc::vals::Adcpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Adcpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Adcpre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Adcpre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Adcpre::DIV6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Adcpre::DIV8 => self * 1u32 / 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Adcpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Adcpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Adcpre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Adcpre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Adcpre::DIV6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Adcpre::DIV8 => self * 8u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 1u32 / 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 1u32 / 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 1u32 / 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 1u32 / 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 1u32 / 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Hpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Hpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Hpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => self * 16u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => self * 64u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => self * 128u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => self * 256u32 / 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => self * 512u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllmul> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllmul) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllmul::MUL2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Pllmul::MUL3 => self * 1u32 / 3u32,
            crate::pac::rcc::vals::Pllmul::MUL4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Pllmul::MUL5 => self * 1u32 / 5u32,
            crate::pac::rcc::vals::Pllmul::MUL6 => self * 1u32 / 6u32,
            crate::pac::rcc::vals::Pllmul::MUL7 => self * 1u32 / 7u32,
            crate::pac::rcc::vals::Pllmul::MUL8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Pllmul::MUL9 => self * 1u32 / 9u32,
            crate::pac::rcc::vals::Pllmul::MUL10 => self * 1u32 / 10u32,
            crate::pac::rcc::vals::Pllmul::MUL11 => self * 1u32 / 11u32,
            crate::pac::rcc::vals::Pllmul::MUL12 => self * 1u32 / 12u32,
            crate::pac::rcc::vals::Pllmul::MUL13 => self * 1u32 / 13u32,
            crate::pac::rcc::vals::Pllmul::MUL14 => self * 1u32 / 14u32,
            crate::pac::rcc::vals::Pllmul::MUL15 => self * 1u32 / 15u32,
            crate::pac::rcc::vals::Pllmul::MUL16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllmul> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllmul) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllmul::MUL2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL3 => self * 3u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL5 => self * 5u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL6 => self * 6u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL7 => self * 7u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL9 => self * 9u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL10 => self * 10u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL11 => self * 11u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL12 => self * 12u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL13 => self * 13u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL14 => self * 14u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL15 => self * 15u32 / 1u32,
            crate::pac::rcc::vals::Pllmul::MUL16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Pllxtpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Pllxtpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllxtpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllxtpre::DIV2 => self * 1u32 / 2u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Pllxtpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Pllxtpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Pllxtpre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Pllxtpre::DIV2 => self * 2u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 1u32 / 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 1u32 / 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 1u32 / 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 1u32 / 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Ppre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Ppre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Ppre::DIV1 => self * 1u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => self * 2u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => self * 4u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => self * 8u32 / 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => self * 16u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Div<crate::pac::rcc::vals::Usbpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn div(self, rhs: crate::pac::rcc::vals::Usbpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Usbpre::DIV1_5 => self * 2u32 / 3u32,
            crate::pac::rcc::vals::Usbpre::DIV1 => self * 1u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::ops::Mul<crate::pac::rcc::vals::Usbpre> for crate::time::Hertz {
    type Output = crate::time::Hertz;
    fn mul(self, rhs: crate::pac::rcc::vals::Usbpre) -> Self::Output {
        match rhs {
            crate::pac::rcc::vals::Usbpre::DIV1_5 => self * 3u32 / 2u32,
            crate::pac::rcc::vals::Usbpre::DIV1 => self * 1u32 / 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_2;
    }
    pub mod ADC12_COMMON {}
    pub mod ADC2 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC1_2;
    }
    pub mod AFIO {}
    pub mod BKP {}
    pub mod CAN {
        pub type RX0 = crate::interrupt::typelevel::USB_LP_CAN1_RX0;
        pub type RX1 = crate::interrupt::typelevel::CAN1_RX1;
        pub type SCE = crate::interrupt::typelevel::CAN1_SCE;
        pub type TX = crate::interrupt::typelevel::USB_HP_CAN1_TX;
    }
    pub mod CRC {}
    pub mod DBGMCU {}
    pub mod DMA1 {
        pub type CH1 = crate::interrupt::typelevel::DMA1_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_CHANNEL2;
        pub type CH3 = crate::interrupt::typelevel::DMA1_CHANNEL3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_CHANNEL4;
        pub type CH5 = crate::interrupt::typelevel::DMA1_CHANNEL5;
        pub type CH6 = crate::interrupt::typelevel::DMA1_CHANNEL6;
        pub type CH7 = crate::interrupt::typelevel::DMA1_CHANNEL7;
    }
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI9_5;
    }
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOE {}
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1_ER;
        pub type EV = crate::interrupt::typelevel::I2C1_EV;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2_ER;
        pub type EV = crate::interrupt::typelevel::I2C2_EV;
    }
    pub mod IWDG {}
    pub mod PWR {}
    pub mod RCC {
        pub type GLOBAL = crate::interrupt::typelevel::RCC;
    }
    pub mod RTC {
        pub type ALARM = crate::interrupt::typelevel::RTC_ALARM;
        pub type SSRU = crate::interrupt::typelevel::RTC;
        pub type STAMP = crate::interrupt::typelevel::RTC;
        pub type TAMP = crate::interrupt::typelevel::TAMPER;
        pub type WKUP = crate::interrupt::typelevel::RTC;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM;
        pub type UP = crate::interrupt::typelevel::TIM1_UP;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3;
        pub type CC = crate::interrupt::typelevel::TIM3;
        pub type COM = crate::interrupt::typelevel::TIM3;
        pub type TRG = crate::interrupt::typelevel::TIM3;
        pub type UP = crate::interrupt::typelevel::TIM3;
    }
    pub mod TIM4 {
        pub type BRK = crate::interrupt::typelevel::TIM4;
        pub type CC = crate::interrupt::typelevel::TIM4;
        pub type COM = crate::interrupt::typelevel::TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM4;
        pub type UP = crate::interrupt::typelevel::TIM4;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2;
    }
    pub mod USART3 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3;
    }
    pub mod USB {
        pub type HP = crate::interrupt::typelevel::USB_HP_CAN1_TX;
        pub type LP = crate::interrupt::typelevel::USB_LP_CAN1_RX0;
        pub type WKUP = crate::interrupt::typelevel::USBWAKEUP;
    }
    pub mod USBRAM {}
    pub mod WWDG {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH1, 0u8);
dma_channel_impl!(DMA1_CH2, 1u8);
dma_channel_impl!(DMA1_CH3, 2u8);
dma_channel_impl!(DMA1_CH4, 3u8);
dma_channel_impl!(DMA1_CH5, 4u8);
dma_channel_impl!(DMA1_CH6, 5u8);
dma_channel_impl!(DMA1_CH7, 6u8);
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL1() {
    <crate::peripherals::DMA1_CH1 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL2() {
    <crate::peripherals::DMA1_CH2 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL3() {
    <crate::peripherals::DMA1_CH3 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL4() {
    <crate::peripherals::DMA1_CH4 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL5() {
    <crate::peripherals::DMA1_CH5 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL6() {
    <crate::peripherals::DMA1_CH6 as crate::dma::ChannelInterrupt>::on_irq();
}
#[cfg(feature = "rt")]
#[crate::interrupt]
unsafe fn DMA1_CHANNEL7() {
    <crate::peripherals::DMA1_CH7 as crate::dma::ChannelInterrupt>::on_irq();
}
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 0usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 1usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 2usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 3usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 4usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 5usize,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::DMA1),
        num: 6usize,
    },
];
pub fn gpio_block(n: usize) -> crate::pac::gpio::Gpio {
    unsafe { crate::pac::gpio::Gpio::from_ptr((1073809408usize + 1024usize * n) as _) }
}
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 131072usize;
pub const WRITE_SIZE: usize = 4usize;
