#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  STM32L4x6
ioregs!(CRC = {
    0x00 => reg32 dr {
        0..31 => dr : rw,
    },
    
    0x04 => reg32 idr {
        0..7 => idr : rw,
    },
    
    0x08 => reg32 cr {
        0 => reset : wo {
            0 => Dummy, // rustc --explain E0083
            1 => Reset,
        },
        3..4 => polysize : rw {
            0 => P32,
            1 => P16,
            2 => P8,
            3 => P7,
        },
        5..6 => rev_in : rw {
            0 => NotAffected,
            1 => ReverseByByte,
            2 => ReverseByHalfWord,
            3 => ReverseByWord,
        },
        7 => rev_out  : rw {
            0 => NotAffected,
            1 => Reversed
        },
    },
    
    0x10 => reg32 init {
        0..31 => init,
    },
    
    0x14 => reg32 pol {
        0..31 => pol,
    },
});