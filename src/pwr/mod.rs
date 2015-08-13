#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  STM32L4x6
ioregs!(PWR = {
    0x00 => reg32 cr1 {
        0..2 => lpms : rw {
            0 => Stop1MR,
            1 => Stop1LPR,
            2 => Stop2,
            3 => Standby,
            4 => Shutdown,
        },
        8 => dbp : rw {
            0 => Disable,
            1 => Enable,
        },
        9..10 => vos : rw {
            1 => Range1,
            2 => Range2,
        },
        14 => lpr : rw {
            0 => MR,
            1 => LPR,
        },
    },
    
    0x04 => reg32 cr2 {
        0 => pvde : rw {
            0 => Disable,
            1 => Enable,
        },
        1..3 => pls : rw {
            0 => V20,
            1 => V22,
            2 => V24,
            3 => V25,
            4 => V26,
            5 => V28,
            6 => V29,
            7 => VExt,
        },
        4 => pvme1 : rw {
            0 => Disable,
            1 => Enable,
        },
        5 => pvme2 : rw {
            0 => Disable,
            1 => Enable,
        },
        6 => pvme3 : rw {
            0 => Disable,
            1 => Enable,
        },
        7 => pvme4 : rw {
            0 => Disable,
            1 => Enable,
        },
        9 => iosv : rw {
            0 => NotPresent,
            1 => Valid,
        },
        10 => usv : rw {
            0 => NotPresent,
            1 => Valid,
        },
    },
    
    0x08 => reg32 cr3 {
        0 => ewup1 : rw {
            0 => Disable,
            1 => Enable,
        },
        1 => ewup2 : rw {
            0 => Disable,
            1 => Enable,
        },
        2 => ewup3 : rw {
            0 => Disable,
            1 => Enable,
        },
        3 => ewup4 : rw {
            0 => Disable,
            1 => Enable,
        },
        4 => ewup5 : rw {
            0 => Disable,
            1 => Enable,
        },
        8 => rrs : rw {
            0 => Disable,
            1 => Enable,
        },
        10 => apc : rw {
            0 => Disable,
            1 => Enable,
        },
        15 => eiwf  : rw{
            0 => Disable,
            1 => Enable,
        },
    },
    
    0x0c => reg32 cr4 {
        0 => wp1 : rw {
            0 => HighLevel,
            1 => LowLevel,
        },
        1 => wp2 : rw {
            0 => HighLevel,
            1 => LowLevel,
        },
        2 => wp3 : rw {
            0 => HighLevel,
            1 => LowLevel,
        },
        3 => wp4 : rw {
            0 => HighLevel,
            1 => LowLevel,
        },
        4 => wp5 : rw {
            0 => HighLevel,
            1 => LowLevel,
        },
        8 => vbe : rw {
            0 => Disable,
            1 => Enable,
        },
        9 => vbrs : rw {
            0 => R5K,
            1 => R1K5,
        },
    },
    
    0x10 => reg32 sr1 {
        0 => wuf1 : ro,
        1 => wuf2 : ro,
        2 => wuf3 : ro,
        3 => wuf4 : ro,
        4 => wuf5 : ro,
        8 => sbf : ro,
        15 => wufi : ro,
    },
    
    0x14 => reg32 sr2 {
        8 => reglps : ro {
            0 => NotReady,
            1 => Ready,
        },
        9 => reglpf : ro {
            0 => MR,
            1 => LPR,
        },
        10 => vosf : ro {
            0 => Ready,
            1 => Changing,
        },
        11 => pvdo : ro {
            0 => Above,
            1 => Below,
        },
        12 => pvmo1 : ro {
            0 => Above,
            1 => Below,
        },
        13 => pvmo2 : ro {
            0 => Above,
            1 => Below,
        },
        14 => pvmo3 : ro {
            0 => Above,
            1 => Below,
        },
        15 => pvmo4 : ro {
            0 => Above,
            1 => Below,
        },
    },
    
    0x18 => reg32 scr {
        0 => cwuf1 : wo,
        1 => cwuf2 : wo,
        2 => cwuf3 : wo,
        3 => cwuf4 : wo,
        4 => cwuf5 : wo,
        8 => csbf : wo,
    },
    
    0x20 => reg32 pucra {
        // TODO: pu[14] reserved, must be kept at reset value
        0..15 => pu[16] : rw,
    },
    
    0x24 => reg32 pdcra {
        // TODO: pd[13] and pd[15] reserved, must be kept at reset value
        0..15 => pd[16] : rw,
    },
    
    0x28 => reg32 pucrb {
        0..15 => pu[16] : rw,
    },
    
    0x2c => reg32 pdcrb {
        // TODO: pd[4] reserved, must be kept at reset value
        0..15 => pd[16] : rw,
    }
    
    0x30 => reg32 pucrc {
        0..15 => pu[16] : rw,
    },
    
    0x34 => reg32 pdcrc {
        0..15 => pd[16] : rw,
    }
    
    0x38 => reg32 pucrd {
        0..15 => pu[16] : rw,
    },
    
    0x3c => reg32 pdcrd {
        0..15 => pd[16] : rw,
    }
    
    0x40 => reg32 pucre {
        0..15 => pu[16] : rw,
    },
    
    0x44 => reg32 pdcre {
        0..15 => pd[16] : rw,
    }
    
    0x48 => reg32 pucrf {
        0..15 => pu[16] : rw,
    },
    
    0x4c => reg32 pdcrf {
        0..15 => pd[16] : rw,
    }
    
    0x50 => reg32 pucrg {
        0..15 => pu[16] : rw,
    },
    
    0x54 => reg32 pdcrg {
        0..15 => pd[16] : rw,
    }
    
    0x58 => reg32 pucrh {
        0..1 => pu[2] : rw,
    },
    
    0x5c => reg32 pdcrh {
        0..1 => pd[2] : rw,
    }

});
