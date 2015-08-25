#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  STM32F4
ioregs!(USART = {
    0x00 => reg32 sr {
        0 => pe : ro,
        1 => fe : ro,
        2 => nf : ro,
        3 => ore : ro,
        4 => idle : ro,
        5 => rxne : rw,
        6 => tc : rw,
        7 => txe : ro,
        8 => lbd : rw,
        9 => cts : rw,
    },
    0x04 => reg32 dr {
        0..8 => dr : rw,
    },
    0x08 => reg32 brr {
        0..3 => div_fraction : rw,
        4..15 => div_mantissa : rw,
    },
    0x0c => reg32 cr1 {
        0 => sbk : rw,
        1 => rwu : rw,
        2 => re : rw,
        3 => te : rw,
        4 => idleie : rw,
        5 => rxneie : rw,
        6 => tcie : rw,
        7 => txeie : rw,
        8 => peie : rw,
        9 => ps : rw,
        10 => pce : rw,
        11 => wake : rw,
        12 => m : rw,
        13 => ue : rw,
        15 => over8 : rw,
    },
    0x10 => reg32 cr2 {
        0..3 => add : rw,
        5 => lbdl : rw,
        6 => lbdie : rw,
        8 => lbcl : rw,
        9 => cpha : rw,
        10 => cpol : rw,
        11 => clken : rw,
        12..13 => stop : rw,
        14 => linen : rw,
    },
    0x14 => reg32 cr3 {
        0 => eie : rw,
        1 => iren : rw,
        2 => irlp : rw,
        3 => hdsel : rw,
        4 => nack : rw,
        5 => scen : rw,
        6 => dmar : rw,
        7 => dmat : rw,
        8 => rtse : rw,
        9 => ctse : rw,
        10 => ctsie : rw,
        11 => onebit : rw,
    },
    0x18 => reg32 gtpr {
        0..7 => psc : rw,
        8..15 => gt : rw,
    },
});
