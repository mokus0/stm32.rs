#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  [RM0090] STM32F4
//  [RM0351] STM32L4x6
//
// Known not to apply to:
//  [RM0091] STM32F0x1, STM32F0x2, STM32F0x8
ioregs!(RNG = {
    0x00 => reg32 cr {
        2 => rngen : rw,
        3 => ie : rw,
    },
    0x04 => reg32 sr {
        0 => drdy : ro,
        1 => cecs : ro,
        2 => secs : ro,
        5 => ceis : rw,
        6 => seis : rw,
    },
    0x08 => reg32 dr {
        0..31 => rndata : ro,
    },
});
