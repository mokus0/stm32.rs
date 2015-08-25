#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to:
//  [RM0090] STM32F4
//  [RM0091] STM32F0x1, STM32F0x2, STM32F0x8
//  [RM0351] STM32L4x6
ioregs!(RTC = {
    0x00 => reg32 tr {
        0..3 => su : rw,
        4..6 => st : rw,
        8..11 => mnu : rw,
        12..14 => mnt : rw,
        16..19 => hu : rw,
        20..21 => ht : rw,
        22 => pm : rw,
    },
    0x04 => reg32 dr {
        0..3 => du : rw,
        4..5 => dt : rw,
        8..11 => mu : rw,
        12 => mt : rw,
        13..15 => wdu : rw,
        16..19 => yu : rw,
        20..23 => yt : rw,
    },
    0x08 => reg32 cr {
        0..2 => wucksel : rw,
        3 => tsedge : rw,
        4 => refckon : rw,
        5 => bypshad : rw,
        6 => fmt : rw,
        // Not in STM32F0x1 et al
        // Not in STM32L4x6 et al
        7 => dce : rw,
        8 => alrae : rw,
        // Not in STM32F0x1 et al
        9 => alrbe : rw,
        10 => wute : rw,
        11 => tse : rw,
        12 => alraie : rw,
        // Not in STM32F0x1 et al
        13 => alrbie : rw,
        14 => wutie : rw,
        15 => tsie : rw,
        16 => add1h : wo,
        17 => sub1h : wo,
        18 => bkp : rw,
        19 => cosel : rw,
        20 => pol : rw,
        21..22 => osel : rw,
        23 => coe : rw,
        // Not in STM32F0x1 et al
        // Not in STM32F4
        24 => itse : rw,
    },
    0x0c => reg32 isr {
        0 => alrawf : ro,
        // Not in STM32F0x1 et al
        1 => alrbwf : ro,
        2 => wutwf : ro,
        3 => shpf : ro,
        4 => inits : ro,
        5 => rsf : rw,
        6 => initf : ro,
        7 => init : rw,
        8 => alraf : rw,
        // Not in STM32F0x1 et al
        9 => alrbf : rw,
        10 => wutf : rw,
        11 => tsf : rw,
        12 => tsovf : rw,
        13 => tamp1f : rw,
        14 => tamp2f : rw,
        // Not in STM32F4
        15 => tamp3f : rw,
        16 => recalpf : ro,
        // Not in STM32F0x1 et al
        // Not in STM32F04
        17 => itsf : rw,
    },
    0x10 => reg32 prer {
        0..14 => prediv_s : rw,
        16..22 => prediv_a : rw,
    },
    0x14 => reg32 wutr {
        0..15 => wut : rw,
    },
    0x1c => reg32 alrmar {
        0..3 => su,
        4..6 => st,
        7 => msk,
        8..11 => mnu,
        12..14 => mnt,
        15 => msk2,
        16..19 => hu,
        20..21 => ht,
        22 => pm,
        23 => msk3,
        24..27 => du,
        28..29 => dt,
        30 => wdsel,
        31 => msk4,
    },
    // not on STM32F0x1
    0x20 => reg32 alrmbr {
        0..3 => su,
        4..6 => st,
        7 => msk,
        8..11 => mnu,
        12..14 => mnt,
        15 => msk2,
        16..19 => hu,
        20..21 => ht,
        22 => pm,
        23 => msk3,
        24..27 => du,
        28..29 => dt,
        30 => wdsel,
        31 => msk4,
    },
    0x24 => reg32 wpr {
        0..7 => key : wo,
    },
    0x28 => reg32 ssr {
        0..15 => ss : ro,
    },
    0x2c => reg32 shiftr {
        0..14 => subfs : wo,
        31 => add1s : wo,
    },
    0x30 => reg32 tstr {
        0..3 => su : ro,
        4..6 => st : ro,
        8..11 => mnu : ro,
        12..14 => mnt : ro,
        16..19 => hu : ro,
        20..21 => ht : ro,
        22 => pm : ro,
    },
    0x34 => reg32 tsdr {
        0..3 => du : ro,
        4..5 => dt : ro,
        8..11 => mu : ro,
        12 => mt : ro,
        13..15 => wdu : ro,
    },
    0x38 => reg32 tsssr {
        0..15 => ss : ro,
    },
    0x3c => reg32 calr {
        0..8 => calm : rw,
        13 => calw16 : rw,
        14 => calw8 : rw,
        15 => calp : rw,
    },
    // known as tampcr on STM32L4x6
    0x40 => reg32 tafcr {
        0 => tamp1e : rw,
        1 => tamp1trg : rw,
        2 => tampie : rw,
        3 => tamp2e : rw,
        4 => tamp2trg : rw,
        5 => tamp3e : rw, // not on STM32F4
        6 => tamp3trg : rw, // not on STM32F4
        7 => tampts : rw,
        8..10 => tampfreq : rw,
        11..12 => tampflt : rw,
        13..14 => tamprch : rw,
        15 => tamppudis : rw,
        
        // TODO: bits 16..31 device-specific
        
        // STM32L4x6 version:
        // 16 -> tamp1ie,
        // 17 => tamp1noerase,
        // 18 => tamp1mf,
        // 19 => tamp2ie,
        // 20 => tamp2noerase,
        // 21 => tamp2mf,
        // 22 => tamp3ie,
        // 23 => tamp3noerase,
        // 24 => tamp3mf,
        
        // STM32F4 version:
        // 16 => tamp1insel : rw,
        // 17 => tsinsel : rw,
        // 18 => alarmouttype : rw,
        
        // STM32F0x1 version:
        // 18 => pc13value : rw,
        // 19 => pc13mode : rw,
        // 20 => pc14value : rw,
        // 21 => pc14mode : rw,
        // 22 => pc15value : rw,
        // 23 => pc15mode : rw,
    },
    0x44 => reg32 alrmassr {
        0..14 => ss : rw,
        24..27 => maskss : rw,
    },
    // not on STM32F0x1
    0x48 => reg32 alrmbssr {
        0..14 => ss : rw,
        24..27 => maskss : rw,
    },
    // not on STM32F4
    // not on STM32F0x1 et al
    0x4c => reg32 or {
        0 => rtc_alarm_type : rw,
        1 => rtc_out_rmp : rw,
    },
    // TODO: only 5 on STM32F0x1
    // TODO: only 20 on STM32F4
    0x50 => reg32 bkpr[32] {
        0..31 => bkp,
    },
});

