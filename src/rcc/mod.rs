#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  STM32L4x6
ioregs!(RCC = {
    0x00 => reg32 cr {
        0 => msi_on : rw {
            0 => Off,
            1 => On,
        },
        1 => msi_rdy : ro {
            0 => NotReady,
            1 => Ready,
        },
        2 => msi_pll_en : rw {
            0 => Off,
            1 => On,
        },
        3 => msi_rgsel : rw,
        4..7 => msi_range : rw,
        8 => hsi_on : rw {
            0 => Off,
            1 => On,
        },
        9 => hsi_ker_on : rw {
            0 => Off,
            1 => On
        },
        10 => hsi_rdy : ro {
            0 => NotReady,
            1 => Ready,
        },
        11 => hsi_asfs : rw {
            0 => Disable,
            1 => Enable,
        },
        16 => hse_on : rw {
            0 => Off,
            1 => On,
        },
        17 => hse_rdy : ro {
            0 => NotReady,
            1 => Ready,
        },
        18 => hse_byp : rw {
            0 => Off,
            1 => On,
        },
        19 => css_on {
            0 => Off,
            1 => On,
        },
        24 => pll_on : rw {
            0 => Off,
            1 => On,
        },
        25 => pll_rdy : ro {
            0 => Unlocked,
            1 => Locked,
        },
        26 => pll_sai1_on : rw {
            0 => Off,
            1 => On,
        },
        27 => pll_sai1_rdy : ro {
            0 => Unlocked,
            1 => Locked,
        },
        28 => pll_sai2_on : rw {
            0 => Off,
            1 => On,
        },
        29 => pll_sai2_rdy : ro {
            0 => Unlocked,
            1 => Locked,
        },
    },
    
    0x04 => reg32 icscr {
        0..7 => msi_cal : ro,
        8..15 => msi_trim : rw,
        16..23 => hsi_cal : ro,
        24..28 => hsi_trim : rw,
    },
    
    0x08 => reg32 cfgr {
        0..1 => sw : rw {
            0 => MSI,
            1 => HSI16,
            2 => HSE,
            3 => PLL,
        },
        2..3 => sws : ro {
            0 => MSI,
            1 => HSI16,
            2 => HSE,
            3 => PLL,
        },
        4..7 => hpre : rw {
            0x0 => Div1,
            0x8 => Div2,
            0x9 => Div4,
            0xA => Div8,
            0xB => Div16,
            0xC => Div64,
            0xD => Div128,
            0xE => Div256,
            0xF => Div512,
        },
        8..10 => ppre1 : rw {
            0 => Div1,
            4 => Div2,
            5 => Div4,
            6 => Div8,
            7 => Div16,
        },
        11..13 => ppre2 : rw {
            0 => Div1,
            4 => Div2,
            5 => Div4,
            6 => Div8,
            7 => Div16,
        },
        15 => stop_wu_ck : rw {
            0 => MSI,
            1 => HSI16,
        },
        24..26 => mcosel : rw {
            0 => Disable,
            1 => SysClk,
            2 => MSI,
            3 => HSI16,
            4 => HSE,
            5 => MainPLL,
            6 => LSI,
            7 => LSE,
        },
        28..30 => mcopre : rw {
            0 => Div1,
            1 => Div2,
            2 => Div4,
            3 => Div8,
            4 => Div16,
        },
    },
    
    0x0c => reg32 pll_cfgr {
        0..1 => pll_src : rw {
            0 => Disable,
            1 => MSI,
            2 => HSI16,
            3 => HSE,
        },
        4..6 => pll_m : rw {
            0 => Div1,
            1 => Div2,
            2 => Div3,
            3 => Div4,
            4 => Div5,
            5 => Div6,
            6 => Div7,
            7 => Div8,
        },
        8..14 => pll_n : rw,
        16 => pll_p_en : rw {
            0 => Disable,
            1 => Enable,
        },
        17 => pll_p : rw {
            0 => Div7,
            1 => Div17,
        },
        20 => pll_q_en : rw {
            0 => Disable,
            1 => Enable,
        },
        21..22 => pll_q : rw {
            0 => Div2,
            1 => Div4,
            2 => Div6,
            3 => Div8,
        },
        24 => pll_r_en : rw {
            0 => Disable,
            1 => Enable,
        },
        25..26 => pll_r : rw {
            0 => Div2,
            1 => Div4,
            2 => Div6,
            3 => Div8,
        },
    },
    
    0x10 => reg32 pll_sai1_cfgr {
        8..14 => pll_sai1_n : rw,
        16 => pll_sai1_p_en : rw {
            0 => Disable,
            1 => Enable,
        },
        17 => pll_sai1_p : rw {
            0 => Div7,
            1 => Div17,
        },
        20 => pll_sai1_q_en : rw {
            0 => Disable,
            1 => Enable,
        },
        21..22 => pll_sai1_q : rw {
            0 => Div2,
            1 => Div4,
            2 => Div6,
            3 => Div8,
        },
        24 => pll_sai1_r_en : rw {
            0 => Disable,
            1 => Enable,
        },
        25..26 => pll_sai1_r : rw {
            0 => Div2,
            1 => Div4,
            2 => Div6,
            3 => Div8,
        },
    },
    
    0x14 => reg32 pll_sai2_cfgr {
        8..14 => pll_sai2_n : rw,
        16 => pll_sai2_p_en : rw {
            0 => Disable,
            1 => Enable,
        },
        17 => pll_sai2_p : rw {
            0 => Div7,
            1 => Div17,
        },
        24 => pll_sai2_r_en : rw {
            0 => Disable,
            1 => Enable,
        },
        25..26 => pll_sai2_r : rw {
            0 => Div2,
            1 => Div4,
            2 => Div6,
            3 => Div8,
        },
    },
    
    0x18 => reg32 cier {
        0 => lsi_rdyie : rw,
        1 => lse_rdyie : rw,
        2 => msi_rdyie : rw,
        3 => hsi_rdyie : rw,
        4 => hse_rdyie : rw,
        5 => pll_rdyie : rw,
        6 => pll_sai1_rdyie : rw,
        7 => pll_sai2_rdyie : rw,
        9 => lse_cssie : rw,
    },
    
    0x1c => reg32 cifr {
        0 => lsi_rdyf : ro,
        1 => lse_rdyf : ro,
        2 => msi_rdyf : ro,
        3 => hsi_rdyf : ro,
        4 => hse_rdyf : ro,
        5 => pll_rdyf : ro,
        6 => pll_sai1_rdyf : ro,
        7 => pll_sai2_rdyf : ro,
        8 => cssf : ro,
        9 => lse_cssf : ro,
    },
    
    0x20 => reg32 cicr {
        0 => lsi_rdyc : wo,
        1 => lse_rdyc : wo,
        2 => msi_rdyc : wo,
        3 => hsi_rdyc : wo,
        4 => hse_rdyc : wo,
        5 => pll_rdyc : wo,
        6 => pll_sai1_rdyc : wo,
        7 => pll_sai2_rdyc : wo,
        8 => cssc : wo,
        9 => lse_cssc : wo,
    },
    
    0x28 => reg32 ahb1_rstr {
        0 => dma1_rst : rw,
        1 => dma2_rst : rw,
        8 => flash_rst : rw,
        12 => crc_rst : rw,
        16 => tsc_rst : rw,
    },
    
    0x2c => reg32 ahb2_rstr {
        0 => gpioa_rst : rw,
        1 => gpiob_rst : rw,
        2 => gpioc_rst : rw,
        3 => gpiod_rst : rw,
        4 => gpioe_rst : rw,
        5 => gpiof_rst : rw,
        6 => gpiog_rst : rw,
        7 => gpioh_rst : rw,
        12 => otgfs_rst : rw,
        13 => adc_rst : rw,
        16 => aes_rst : rw,
        18 => rng_rst : rw,
    },
    
    0x30 => reg32 ahb3_rstr {
        0 => fmc_rst : rw,
        8 => qspi_rst : rw,
    },
    
    0x38 => reg32 apb1_rstr1 {
        0 => tim2_rst : rw,
        1 => tim3_rst : rw,
        2 => tim4_rst : rw,
        3 => tim5_rst : rw,
        4 => tim6_rst : rw,
        5 => tim7_rst : rw,
        9 => lcd_rst : rw,
        14 => spi2_rst : rw,
        15 => spi3_rst : rw,
        17 => usart2_rst : rw,
        18 => usart3_rst : rw,
        19 => uart4_rst : rw,
        20 => uart5_rst : rw,
        21 => i2c1_rst : rw,
        22 => i2c2_rst : rw,
        23 => i2c3_rst : rw,
        25 => can1_rst : rw,
        28 => pwr_rst : rw,
        29 => dac1_rst : rw,
        30 => opamp_rst : rw,
        31 => lptim1_rst : rw,
    },
    
    0x3c => reg32 apb1_rstr2 {
        0 => lpuart1_rst : rw,
        2 => swpmi_rst : rw,
        5 => lptim2_rst : rw,
    },
    
    0x40 => reg32 apb2_rstr {
        0 => syscfg_rst : rw,
        10 => sdmmc1_rst : rw,
        11 => tim1_rst : rw,
        12 => spi1_rst : rw,
        13 => tim8_rst : rw,
        14 => usart1_rst : rw,
        16 => tim15_rst : rw,
        17 => tim16_rst : rw,
        18 => tim17_rst : rw,
        21 => sai1_rst : rw,
        22 => sai2_rst : rw,
        24 => dfsdm_rst : rw,
    },
    
    0x48 => reg32 ahb1enr {
        0 => dma1_en : rw,
        1 => dma2_en : rw,
        8 => flash_en : rw,
        12 => crc_en : rw,
        16 => tsc_en : rw,
    },
    
    0x4c => reg32 ahb2enr {
        0 => gpioa_en : rw,
        1 => gpiob_en : rw,
        2 => gpioc_en : rw,
        3 => gpiod_en : rw,
        4 => gpioe_en : rw,
        5 => gpiof_en : rw,
        6 => gpiog_en : rw,
        7 => gpioh_en : rw,
        12 => otgfs_en : rw,
        13 => adc_en : rw,
        16 => aes_en : rw,
        18 => rng_en : rw,
    },
    
    0x50 => reg32 ahb3enr {
        0 => fmc_en : rw,
        8 => qspi_en : rw,
    },
    
    0x58 => reg32 apb1enr1 {
        0 => tim2_en : rw,
        1 => tim3_en : rw,
        2 => tim4_en : rw,
        3 => tim5_en : rw,
        4 => tim6_en : rw,
        5 => tim7_en : rw,
        9 => lcd_en : rw,
        11 => wwdg_en : rw,
        14 => spi2_en : rw,
        15 => spi3_en : rw,
        17 => usart2_en : rw,
        18 => usart3_en : rw,
        19 => uart4_en : rw,
        20 => uart5_en : rw,
        21 => i2c1_en : rw,
        22 => i2c2_en : rw,
        23 => i2c3_en : rw,
        25 => can1_en : rw,
        28 => pwr_en : rw,
        29 => dac1_en : rw,
        30 => opamp_en : rw,
        31 => lptim1_en : rw,
    },
    
    0x5c => reg32 apb1enr2 {
        0 => lpuart1_en : rw,
        2 => swpmi_en : rw,
        5 => lptim2_en : rw,
    },
    
    0x60 => reg32 apb2enr {
        0 => syscfg_en : rw,
        7 => fw_en : rw,
        10 => sdmmc1_en : rw,
        11 => tim1_en : rw,
        12 => spi1_en : rw,
        13 => tim8_en : rw,
        14 => usart1_en : rw,
        16 => tim15_en : rw,
        17 => tim16_en : rw,
        18 => tim17_en : rw,
        21 => sai1_en : rw,
        22 => sai2_en : rw,
        24 => dfsdm_en : rw,
    },
    
    0x68 => reg32 ahb1smenr {
        0 => dma1_smen : rw,
        1 => dma2_smen : rw,
        8 => flash_smen : rw,
        9 => sram1_smen : rw,
        12 => crc_smen : rw,
        16 => tsc_smen : rw,
    },
    
    0x6c => reg32 ahm2smenr {
        0 => gpioa_smen : rw,
        1 => gpiob_smen : rw,
        2 => gpioc_smen : rw,
        3 => gpiod_smen : rw,
        4 => gpioe_smen : rw,
        5 => gpiof_smen : rw,
        6 => gpiog_smen : rw,
        7 => gpioh_smen : rw,
        9 => sram2_smen : rw,
        12 => otgfs_smen : rw,
        13 => adc_smen : rw,
        16 => aes_smen : rw,
        18 => rng_smen : rw,
    },
    
    0x70 => reg32 ahm3smenr {
        0 => fmc_smen,
        8 => qspi_smen,
    },
    
    0x78 => reg32 apb1smenr1 {
        0 => tim2_smen : rw,
        1 => tim3_smen : rw,
        2 => tim4_smen : rw,
        3 => tim5_smen : rw,
        4 => tim6_smen : rw,
        5 => tim7_smen : rw,
        9 => lcd_smen : rw,
        11 => wwdg_smen : rw,
        14 => spi2_smen : rw,
        15 => spi3_smen : rw,
        17 => usart2_smen : rw,
        18 => usart3_smen : rw,
        19 => uart4_smen : rw,
        20 => uart5_smen : rw,
        21 => i2c1_smen : rw,
        22 => i2c2_smen : rw,
        23 => i2c3_smen : rw,
        25 => can1_smen : rw,
        28 => pwr_smen : rw,
        29 => dac1_smen : rw,
        30 => opamp_smen : rw,
        31 => lptim1_smen : rw,
    },
    
    0x7c => reg32 apb1smenr2 {
        0 => lpuart1_smen : rw,
        2 => swpmi_smen : rw,
        5 => lptim2_smen : rw,
    },
    
    0x80 => reg32 apb2smenr {
        0 => syscfg_smen : rw,
        10 => sdmmc1_smen : rw,
        11 => tim1_smen : rw,
        12 => spi1_smen : rw,
        13 => tim8_smen : rw,
        14 => usart1_smen : rw,
        16 => tim15_smen : rw,
        17 => tim16_smen : rw,
        18 => tim17_smen : rw,
        21 => sai1_smen : rw,
        22 => sai2_smen : rw,
        24 => dfsdm_smen : rw,
    },
    
    0x88 => reg32 ccipr {
        // TODO: lots of enums
        0..1 => usart1_sel : rw,
        2..3 => usart2_sel : rw,
        4..5 => usart3_sel : rw,
        6..7 => uart4_sel : rw,
        8..9 => uart5_sel : rw,
        10..11 => lpuart1_sel : rw,
        12..13 => i2c1_sel : rw,
        14..15 => i2c2_sel : rw,
        16..17 => i2c3_sel : rw,
        18..19 => lptim1_sel : rw,
        20..21 => lptim2_sel : rw,
        22..23 => sai1_sel : rw,
        24..25 => sai2_sel : rw,
        26..27 => clk48_sel : rw,
        28..29 => adc_sel : rw,
        30 => swpmi_sel : rw,
        31 => dfsdm_sel : rw,
    },
    
    0x90 => reg32 bdcr {
        0 => lse_on : rw {
            0 => Off,
            1 => On,
        },
        1 => lse_rdy : ro {
            0 => NotReady,
            1 => Ready,
        },
        2 => lse_byp : rw {
            0 => NotBypassed,
            1 => Bypassed,
        },
        3..4 => lse_drv : rw {
            0 => Lower,
            1 => MediumLow,
            2 => MediumHigh,
            3 => Higher,
        },
        5 => lse_css_on : rw {
            0 => Off,
            1 => On,
        },
        6 => lse_css_d : ro {
            0 => OK,
            1 => Failure,
        },
        8..9 => rtc_sel : rw {
            0 => NoClock,
            1 => LSE,
            2 => LSI,
            3 => HSE,
        },
        15 => rtc_en : rw {
            0 => Disable,
            1 => Enable,
        },
        16 => bd_rst : rw,
        24 => lsco_en : rw {
            0 => Disable,
            1 => Enable,
        },
        25 => lsco_sel : rw {
            0 => LSI,
            1 => LSE,
        },
    },
    
    0x94 => reg32 csr {
        0 => lsi_on : rw {
            0 => Off,
            1 => On,
        },
        1 => lsi_rdy : ro {
            0 => NotReady,
            1 => Ready,
        },
        8..11 => msi_srange : rw,
        23 => rmvf : rw,
        24 => fw_rstf : ro,
        25 => obl_rstf : ro,
        26 => pin_rstf : ro,
        27 => bor_rstf : ro,
        28 => sft_rstf : ro,
        29 => iwdg_rstf : ro,
        30 => wwdg_rstf : ro,
        31 => lpwr_rstf : ro,
    }
});