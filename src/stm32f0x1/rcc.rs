#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
// [RM0091] STM32F0x1, STM32F0x2, STM32F0x8
// [RM0360] STM32F030x4/6/8/C and STM32F070x6/B
ioregs!(RCC = {
    0x00 => reg32 cr {
        0 => hsi_on : rw,
        1 => hsi_rdy : ro,
        3..7 => hsi_trim : rw,
        8..15 => hsi_cal : ro,
        16 => hse_on : rw,
        17 => hse_rdy : ro,
        18 => hse_byp : rw,
        19 => css_on : rw,
        24 => pll_on : rw,
        25 => pll_rdy : ro,
    },
    0x04 => reg32 cfgr {
        0..1 => sw : rw,
        2..3 => sws : ro,
        4..7 => hpre : rw,
        8..10 => ppre : rw,
        14 => adc_pre : rw,
        15..16 => pll_src : rw,     // TODO: only bit 16 on STM32F030x4 et al
        17 => pll_xtpre : rw,
        18..21 => pll_mul : rw,
        24..27 => mco : rw,
        28..30 => mco_pre : rw,
        31 => pll_nodiv : rw,
    },
    0x08 => reg32 cir {
        0 => lsi_rdyf : ro,
        1 => lse_rdyf : ro,
        2 => hsi_rdyf : ro,
        3 => hse_rdyf : ro,
        4 => pll_rdyf : ro,
        5 => hsi14_rdyf : ro,
        6 => hsi48_rdyf : ro, // TODO: not on STM32F030x4, et al
        7 => cssf : ro,
        8 => lsi_rdyie : rw,
        9 => lse_rdyie : rw,
        10 => hsi_rdyie : rw,
        11 => hse_rdyie : rw,
        12 => pll_rdyie : rw,
        13 => hsi14_rdyie : rw,
        14 => hsi48_rdyie : rw, // not on STM32F030x4 et al
        16 => lsi_rdyc : wo,
        17 => lse_rdyc : wo,
        18 => hsi_rdyc : wo,
        19 => hse_rdyc : wo,
        20 => pll_rdyc : wo,
        21 => hsi14_rdyc : wo,
        22 => hsi48_rdyc : wo, // not on STM32F030x4 et al
        23 => cssc : wo,
    },
    0x0c => reg32 apb2rstr {
        0 => syscfg_rst : rw,
        5 => usart6_rst : rw,
        6 => usart7_rst : rw, // not on STM32F030x4
        7 => usart8_rst : rw, // not on STM32F030x4
        9 => adc_rst : rw,
        11 => tim1_rst : rw,
        12 => spi1_rst : rw,
        14 => usart1_rst : rw,
        16 => tim15_rst : rw,
        17 => tim16_rst : rw,
        18 => tim17_rst : rw,
        22 => dbg_mcu_rst : rw,
    },
    0x10 => reg32 apb1rstr {
        0 => tim2_rst : rw, // not on STM32F030x4
        1 => tim3_rst : rw,
        4 => tim6_rst : rw,
        5 => tim7_rst : rw,
        8 => tim14_rst : rw,
        11 => wwdg_rst : rw,
        14 => spi2_rst : rw,
        17 => usart2_rst : rw,
        18 => usart3_rst : rw,
        19 => usart4_rst : rw,
        20 => usart5_rst : rw,
        21 => i2c1_rst : rw,
        22 => i2c2_rst : rw,
        23 => usb_rst : rw,
        25 => can_rst : rw, // not on STM32F030x4
        27 => crs_rst : rw, // not on STM32F030x4
        28 => pwr_rst : rw,
        29 => dac_rst : rw, // not on STM32F030x4
        30 => cec_rst : rw, // not on STM32F030x4
    },
    0x14 => reg32 ahbenr {
        0 => dma_en : rw,
        1 => dma2_en : rw, // not on STM32F030x4
        2 => sram_en : rw,
        4 => flitf_en : rw,
        6 => crc_en : rw,
        17 => gpioa_en : rw,
        18 => gpiob_en : rw,
        19 => gpioc_en : rw,
        20 => gpiod_en : rw,
        21 => gpioe_en : rw, // not on STM32F030x4
        22 => gpiof_en : rw,
        24 => tsc_en : rw, // not on STM32F030x4
    },
    0x18 => reg32 apb2enr {
        0 => syscfg_comp_en : rw,
        5 => usart6_en : rw,
        6 => usart7_en : rw, // not on STM32F030x4
        7 => usart8_en : rw, // not on STM32F030x4
        9 => adc_en : rw,
        11 => tim1_en : rw,
        12 => spi1_en : rw,
        14 => usart1_en : rw,
        16 => tim15_en : rw,
        17 => tim16_en : rw,
        18 => tim17_en : rw,
        22 => dbg_mcu_en : rw,
    },
    0x1c => reg32 apb1enr {
        0 => tim2_en : rw, // not on STM32F030x4
        1 => tim3_en : rw,
        4 => tim6_en : rw,
        5 => tim7_en : rw,
        8 => tim14_en : rw,
        11 => wwdg_en : rw,
        14 => spi2_en : rw,
        17 => usart2_en : rw,
        18 => usart3_en : rw,
        19 => usart4_en : rw,
        20 => usart5_en : rw,
        21 => i2c1_en : rw,
        22 => i2c2_en : rw,
        23 => usb_en : rw,
        25 => can_en : rw, // not on STM32F030x4
        27 => crs_en : rw, // not on STM32F030x4
        28 => pwr_en : rw,
        29 => dac_en : rw, // not on STM32F030x4
        30 => cec_en : rw, // not on STM32F030x4
    },
    0x20 => reg32 bdcr {
        0 => lse_on : rw,
        1 => lse_rdy : ro,
        2 => lse_byp : rw,
        3..4 => lse_drv : rw,
        8..9 => rtc_sel : rw,
        15 => rtc_en : rw,
        16 => bd_rst : rw,
    },
    0x24 => reg32 csr {
        0 => lsi_on : rw,
        1 => lsi_rdy : ro,
        23 => v18pwr_rstf : ro,
        24 => rmvf : wo,
        25 => obl_rstf : ro,
        26 => pin_rstf : ro,
        27 => por_rstf : ro,
        28 => sft_rstf : ro,
        29 => iwdg_rstf : ro,
        30 => wwdg_rstf : ro,
        31 => lpwr_rstf : ro,
    },
    0x28 => reg32 ahbrstr {
        17 => gpioa_rst : rw,
        18 => gpiob_rst : rw,
        19 => gpioc_rst : rw,
        20 => gpiod_rst : rw,
        21 => gpioe_rst : rw, // not on STM32F030x4
        22 => gpiof_rst : rw,
        24 => tsc_rst : rw, // not on STM32F030x4
    },
    0x2c => reg32 cfgr2 {
        0..3 => prediv : rw,
    },
    0x30 => reg32 cfgr3 {
        0..1 => usart1_sw : rw,
        4 => i2c1_sw : rw,
        6 => cec_sw : rw, // not on STM32F030x4
        7 => usb_sw : rw,
        8 => adc_sw : rw,
        16..17 => usart2_sw : rw, // not on STM32F030x4
        18..19 => usart3_sw : rw, // not on STM32F030x4
    },
    0x34 => reg32 cr2 {
        0 => hsi14_on : rw,
        1 => hsi14_rdy : ro,
        2 => hsi14_dis : rw,
        3..7 => hsi14_trim : rw,
        8..15 => hsi14_cal : ro,
        16 => hsi48_on : rw, // not on STM32F030x4
        17 => hsi48_rdy : ro, // not on STM32F030x4
        24..31 => hsi48_cal : ro, // not on STM32F030x4
    },
});
