#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  STM32L4x6
ioregs!(FLASH = {
    0x00 => reg32 acr {
        0..2 => latency : rw {
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
        },
        8 => prften : rw {
            0 => Enabled,
            1 => Disabled,
        },
        9 => icen : rw {
            0 => Enabled,
            1 => Disabled,
        },
        10 => dcen : rw {
            0 => Enabled,
            1 => Disabled,
        },
        11 => icrst : rw {
            0 => Reset,
            1 => NotReset
        },
        12 => dcrst : rw {
            0 => Reset,
            1 => NotReset
        },
        13 => run_pd : rw {
            0 => Idle,
            1 => PDown,
        },
        14 => sleep_pd : rw {
            0 => Idle,
            1 => PDown,
        },
    },
    
    0x04 => reg32 pdkeyr {
        0..31 => pdkeyr : wo {
            0x04152637u32 => pdkey1,
            0xFAFBFCFDu32 => pdkey2,
        },
    },
    
    0x08 => reg32 keyr {
        0..31 => keyr : wo {
            0x45670123u32 => key1,
            0xCDEF89ABu32 => key2,
        },
    },
    0x0c => reg32 optkeyr {
        0..31 => optkeyr : wo {
            0x08192A3Bu32 => key1,
            0x4C5D6E7Fu32 => key2,
        },
    },
    
    0x10 => reg32 sr {
        0 => eop : set_to_clear,
        1 => op_err : set_to_clear,
        3 => prog_err : set_to_clear,
        4 => wrp_err : set_to_clear,
        5 => pga_err : set_to_clear,
        6 => siz_err : set_to_clear,
        7 => pgs_err : set_to_clear,
        8 => miss_err : set_to_clear,
        9 => fast_err : set_to_clear,
        14 => rd_err : set_to_clear,
        15 => optv_err : set_to_clear,
        16 => bsy : ro,
    },
    
    0x14 => reg32 cr {
        0 => pg : rw {
            0 => Enabled,
            1 => Disabled,
        },
        1 => per : rw {
            0 => Enabled,
            1 => Disabled,
        },
        2 => mer1 : rw,
        3..10 => pnb : rw,
        11 => bker : rw {
            0 => Bank1,
            1 => Bank2,
        },
        15 => mer2 : rw,
        16 => strt : rw,
        17 => opt_strt : rw,
        18 => fstpg : rw {
            0 => Disabled,
            1 => Enabled,
        },
        24 => eop_ie : rw {
            0 => Disabled,
            1 => Enabled,
        },
        25 => err_ie : rw {
            0 => Disabled,
            1 => Enabled,
        },
        26 => rd_errie : rw {
            0 => Disabled,
            1 => Enabled,
        },
        27 => obl_launch : rw {
            0 => Complete,
            1 => Requested,
        },
        30 => opt_lock : rw {
            0 => Unlocked,
            1 => Locked,
        },
        31 => lock : rw {
            0 => Unlocked,
            1 => Locked,
        },
    },
    
    0x18 => reg32 eccr {
        0..18 => addr_ecc : ro,
        19 => bk_ecc : ro {
            0 => Bank1,
            1 => Bank2,
        },
        20 => sysf_ecc : ro,
        24 => ecc_ie : rw {
            0 => Disabled,
            1 => Enabled,
        },
        30 => eccc : set_to_clear,
        31 => eccd : set_to_clear,
    },
    
    0x20 => reg32 optr {
        0..7 => rdp : rw,
        8..10 => bor_lev : rw,
        12 => nrst_stop : rw {
            0 => Reset,
            1 => NoReset,
        },
        13 => nrst_stdby : rw {
            0 => Reset,
            1 => NoReset,
        },
        14 => nrst_shdw : rw {
            0 => Reset,
            1 => NoReset,
        },
        16 => iwdg_sw : rw {
            0 => Hardware,
            1 => Software,
        },
        17 => iwdg_stop : rw {
            0 => Frozen,
            1 => Running,
        },
        18 => iwdg_stby : rw {
            0 => Frozen,
            1 => Running,
        },
        19 => wwdg_sw : rw {
            0 => Hardware,
            1 => Software,
        },
        20 => bfb2 : rw {
            0 => Disabled,
            1 => Enabled,
        },
        21 => dualbank : rw {
            0 => SingleBank,
            1 => DualBank,
        },
        23 => nboot1 : rw,
        24 => sram2_pe : rw {
            0 => Enabled,
            1 => Disabled,
        },
        25 => sram2_rst : rw {
            0 => Erase,
            1 => Keep,
        },
    },
    
    0x24 => reg32 pcrop1sr {
        0..15 => strt : rw,
    },
    
    0x28 => reg32 pcrop1er {
        0..15 => end : rw,
        31 => rdp : rw {
            0 => DoNotErase,
            1 => Erase,
        },
    },
    
    0x2c => reg32 wrp1ar {
        0..7 => strt : rw,
        16..23 => end : rw,
    },
    
    0x30 => reg32 wrp1br {
        0..7 => strt : rw,
        16..23 => end : rw,
    },
    
    0x44 => reg32 pcrop2sr {
        0..15 => strt : rw,
    },
    
    0x48 => reg32 pcrop2er {
        0..15 => end : rw,
    },
    
    0x4c => reg32 wrp2ar {
        0..7 => strt : rw,
        16..23 => end : rw,
    },
    
    0x50 => reg32 wrp2br {
        0..7 => strt : rw,
        16..23 => end : rw,
    },
});
