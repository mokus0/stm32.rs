#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  STM32L4x6 (TIM2/3/4/5)
ioregs!(GPTIM32 = {
    0x00 => reg32 cr1 {
        0 => cen : rw {
            0 => Disable,
            1 => Enable,
        },
        1 => udis : rw {
            0 => Enable,
            1 => Disable,
        },
        2 => urs : rw {
            0 => Any,
            1 => Limited,
        },
        3 => opm : rw {
            0 => Disable,
            1 => Enable,
        },
        4 => dir : rw {
            0 => Up,
            1 => Down,
        },
        5..6 => cms : rw {
            0 => EdgeAligned,
            1 => CenterAligned1,
            2 => CenterAligned2,
            3 => CenterAligned3,
        },
        7 => arpe : rw {
            0 => NotBuffered,
            1 => Buffered,
        },
        8..9 => ckd : rw {
            0 => Div1,
            1 => Div2,
            2 => Div4,
        },
        11 => uif_remap : rw {
            // TODO: STM32L4x6 only
            0 => Disable,
            1 => Enable,
        },
    },
    
    0x04 => reg32 cr2 {
        3 => ccds : rw {
            0 => CC,
            1 => Update,
        },
        4..6 => mms : rw {
            0 => Reset,
            1 => Enable,
            2 => Update,
            3 => ComparePulse,
            4 => CompareOC1Ref,
            5 => CompareOC2Ref,
            6 => CompareOC3Ref,
            7 => CompareOC4Ref,
        },
        7 => ti1s : rw {
            0 => Normal,
            1 => XOr,
        },
    },
    
    0x08 => reg32 smcr {
        0..2 => sms02 : rw,
        3 => occs : rw {
            // TODO: STM32L4x6 only
            0 => OCRef_Clr,
            1 => ETRF,
        },
        4..6 => ts : rw {
            0 => ITR0, // TODO: reserved on STM32L4x6
            1 => ITR1,
            2 => ITR2,
            3 => ITR3, // TODO: reserved on STM32L4x6
            4 => TI1F_ED,
            5 => TI1FP1,
            6 => TI1FP2,
            7 => ETRF,
        },
        7 => mssm : rw {
            0 => NoAction,
            1 => Synchronized,
        },
        8..11 => etf : rw,
        12..13 => etps : rw {
            1 => Div1,
            2 => Div2,
            3 => Div4,
            4 => Div8,
        },
        14 => ece : rw {
            0 => Disable,
            1 => Enable,
        },
        15 => etp : rw {
            0 => NonInverted,
            1 => Inverted,
        },
        16 => sms3 : rw, // TODO: STM32L4x6 only
    },
    
    0x0c => reg32 dier {
        0 => uie : rw,
        1 => cc1ie : rw,
        2 => cc2ie : rw,
        3 => cc3ie : rw,
        4 => cc4ie : rw,
        6 => tie : rw,
        8 => ude : rw,
        9 => cc1de : rw,
        10 => cc2de : rw,
        11 => cc3de : rw,
        12 => cc4de : rw,
        14 => tde : rw,
    },
    
    0x10 => reg32 sr {
        0 => uif : rw,
        1 => cc1if : rw,
        2 => cc2if : rw,
        3 => cc3if : rw,
        4 => cc4if : rw,
        6 => tif : rw,
        9 => cc1of : rw,
        10 => cc2of : rw,
        11 => cc3of : rw,
        12 => cc4of : rw,
    },
    
    0x14 => reg32 egr {
        0 => ug : wo,
        1 => cc1g : wo,
        2 => cc2g : wo,
        3 => cc3g : wo,
        4 => cc4g : wo,
        6 => tg : wo,
    },
    
    0x18 => reg32 ccmr1 {
        // TODO: ioreg support for custom register types...
        //       this one would map nicely to a pair of 2-variant enums
        // TODO: remember to compare all supported MCUs when revisiting this
        0..31 => ccmr1 : rw,
    },
    
    0x1c => reg32 ccmr2 {
        0..31 => ccmr2 : rw,
    },
    
    0x20 => reg32 ccer {
        // TODO: ioreg support for field groups?
        0 => cc1e : rw,
        1 => cc1p : rw,
        3 => cc1np : rw,
        4 => cc2e : rw,
        5 => cc2p : rw,
        7 => cc2np : rw,
        8 => cc3e : rw,
        9 => cc3p : rw,
        11 => cc3np : rw,
        12 => cc4e : rw,
        13 => cc4p : rw,
        14 => cc4np : rw,
    },
    
    0x24 => reg32 cnt {
        // TODO: note about bit 31 on STM32L4x6 (it has an alternate function)
        // TODO: ioregs support for regs that are just plain values (possibly of parameter types)
        0..31 => cnt : rw,
    },
    
    0x28 => reg32 psc {
        0..15 => psc : rw,
    },
    
    0x2c => reg32 arr {
        0..31 => arr : rw,
    },
    
    0x34 => reg32 ccr[4] {
        0..31 => ccr : rw,
    },
    
    0x48 => reg32 dcr {
        0..4 => dba : rw,
        8..12 => dbl : rw,
    },
    
    0x4c => reg32 dmar {
        0..15 => dmab : rw,
    },
});

// TODO: ioregs support for split registers
#[doc = "Set value of `sms02` field: no documentation"]
impl <'a> GPTIM32_smcr_Update<'a> {
    #[doc = "Set value of `sms3` field: no documentation"]
    pub fn set_sms<'b>(&'b mut self, new_value: GPTIM32_smcr_sms)
     -> &'b mut GPTIM32_smcr_Update<'a> {
        let field_mask = 0b1_00000000_00000111;
        self.value = (self.value & !field_mask) | (new_value as u32);
        self.mask |= field_mask;
        self
    }
}

impl GPTIM32_smcr_Get {
    pub fn sms(&self) -> GPTIM32_smcr_sms {
        match self.value & 0b1_00000000_00000111 {
            0b0_00000000_00000000 => ::core::option::Option::Some(GPTIM32_smcr_sms::Disable),
            0b0_00000000_00000001 => ::core::option::Option::Some(GPTIM32_smcr_sms::Encoder1),
            0b0_00000000_00000010 => ::core::option::Option::Some(GPTIM32_smcr_sms::Encoder2),
            0b0_00000000_00000011 => ::core::option::Option::Some(GPTIM32_smcr_sms::Encoder3),
            0b0_00000000_00000100 => ::core::option::Option::Some(GPTIM32_smcr_sms::Reset),
            0b0_00000000_00000101 => ::core::option::Option::Some(GPTIM32_smcr_sms::Gated),
            0b0_00000000_00000110 => ::core::option::Option::Some(GPTIM32_smcr_sms::Trigger),
            0b0_00000000_00000111 => ::core::option::Option::Some(GPTIM32_smcr_sms::ExtClk1),
            0b1_00000000_00000000 => ::core::option::Option::Some(GPTIM32_smcr_sms::ResetAndTrigger),
            _ => ::core::option::Option::None,
        }.unwrap()
    }
}

impl GPTIM32_smcr {
    #[allow(dead_code, missing_docs)]
    pub fn set_sms<'a>(&'a self, new_value: GPTIM32_smcr_sms)
     -> GPTIM32_smcr_Update<'a> {
        let mut setter: GPTIM32_smcr_Update = GPTIM32_smcr_Update::new(self);
        setter.set_sms(new_value);
        setter
    }
    #[allow(dead_code, missing_docs)]
    pub fn sms(&self) -> GPTIM32_smcr_sms { GPTIM32_smcr_Get::new(self).sms() }
}

#[allow(non_camel_case_types)]
pub enum GPTIM32_smcr_sms {
    Disable         = 0b0_00000000_00000000,
    Encoder1        = 0b0_00000000_00000001,
    Encoder2        = 0b0_00000000_00000010,
    Encoder3        = 0b0_00000000_00000011,
    Reset           = 0b0_00000000_00000100,
    Gated           = 0b0_00000000_00000101,
    Trigger         = 0b0_00000000_00000110,
    ExtClk1         = 0b0_00000000_00000111,
    ResetAndTrigger = 0b1_00000000_00000000,
}
