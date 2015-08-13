#![no_std]

use volatile_cell::VolatileCell;

// TODO: better to represent these page registers as 0..N
// with caveats that low M bits must be 0?

// Known to apply to:
//  STM32L4x6
ioregs!(FW = {
    0x00 => reg32 cssa {
        8..23 => add, 
    },
    
    0x04 => reg32 csl {
        8..21 => leng,
    },
    
    0x08 => reg32 nvdssa {
        8..23 => add,
    },
    
    0x0c => reg32 nvdsl {
        8..21 => leng,
    },
    
    0x10 => reg32 vdssa {
        6..16 => add,
    },
    
    0x14 => reg32 vdsl {
        6..16 => leng,
    },
    
    0x20 => reg32 cr {
        0 => fpa : rw {
            0 => SystemReset,
            1 => CloseFirewall,
        },
        1 => vds : rw {
            0 => NotShared,
            1 => Shared,
        },
        2 => vde : rw {
            0 => NoExecute,
            1 => Execute,
        },
    }
});
