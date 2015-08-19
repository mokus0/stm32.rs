#![no_std]

use volatile_cell::VolatileCell;

// Option registers for TIM2 and TIM5; both start at 0x50 relative to 
// base of their respective timer peripherals
//  (STM32F4)
ioregs!(TIM2_OPT = {
    0x00 => reg32 or {
        10..11 => itr1_rmp,
    },
});

ioregs!(TIM5_OPT = {
    0x00 => reg32 or {
        6..7 => ti4_rmp,
    },
});

