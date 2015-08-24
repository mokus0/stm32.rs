#![no_std]

use volatile_cell::VolatileCell;

// Option registers for TIM2 and TIM3; both start at 0x50 relative to 
// base of their respective timer peripherals
//  (STM32L4x6)
ioregs!(TIM2_OPT = {
    0x00 => reg32 or1 {
        0 => itr1_rmp : rw {
            0 => TIM8_TRGO,
            1 => OTG_FS_SOF,
        },
        1 => etr1_rmp : rw {
            0 => IO,
            1 => LSE,
        },
        2..3 => ti4_rmp : rw {
            0 => IO,
            1 => COMP1_OUT,
            2 => COMP2_OUT,
            3 => Or,
        },
    },
    0x10 => reg32 or2 {
        14..16 => etr_sel {
            0 => LegacyMode,
            1 => COMP1,
            2 => COMP2,
        },
    },
});

ioregs!(TIM3_OPT = {
    0x00 => reg32 or1 {
        0..1 => ti1_rmp : rw {
            0 => IO,
            1 => COMP1_OUT,
            2 => COMP2_OUT,
            3 => Or,
        },
    },
    0x10 => reg32 or2 {
        14..16 => etr_sel {
            0 => LegacyMode,
            1 => COMP1,
        },
    },
});

