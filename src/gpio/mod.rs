#![no_std]

use volatile_cell::VolatileCell;

// Known to apply to (with exceptions noted):
//  STM32F030x4/6/8/C and STM32F070x6/B
//  STM32F0x1, STM32F0x2, STM32F0x8
//  STM32F205xx, STM32F207xx, STM32F215xx and STM32F217xx
//  STM32F301x6/8 and STM32F318x8
//  STM32F302xB/C/D/E and STM32F302x6/8
//  STM32F303xB/C/D/E, STM32F303x6/8, STM32F328x8, STM32F358xC, STM32F398xE
//  STM32F334xx
//  STM32F37xx
//  STM32F4
//  STM32F401xB/C and STM32F401xD/E
//  STM32F411xC/E
//  STM32F75xxx and STM32F74xxx
//  STM32L0x2
//  STM32L0x3
//  STM32L100xx, 151xx, 152xx and 162xx
//  STM32L4x6
//  STM32F446xx
//
// Known not to apply to:
//  STM32F100xx
//  STM32F101xx, 2xx, 3xx, 5xx, 7xx
ioregs!(GPIO = {
    0x00 => reg32 moder {
        0..31 => mode[16] : rw {
            0b00 => Input,
            0b01 => Output,
            0b10 => Alternate,
            0b11 => Analog,
        },
    },
    0x04 => reg32 otyper {
        0..15 => ot[16] : rw {
            0 => PushPull,
            1 => OpenDrain,
        },
        16..31 => reserved : ro,
    },
    0x08 => reg32 ospeedr {
        0..31 => ospeed[16] : rw {
            // TODO: names not common to all devices
            0b00 => Low,
            0b01 => Medium,
            0b10 => Fast,
            0b11 => High,
        }
    },
    0x0c => reg32 pupdr {
        0..31 => pupdr[16] : rw {
            0b00 => None,
            0b01 => PullUp,
            0b10 => PullDown,
            0b11 => Reserved,
        },
    },
    0x10 => reg32 idr {
        0..15 => idr[16] : ro,
        16..31 => reserved : ro,
    },
    0x14 => reg32 odr {
        0..15 => odr[16] : rw,
        16..31 => reserved : ro,
    },
    0x18 => reg32 bsrr {
        0..15 => bs[16] : wo,
        16..31 => br[16] : wo,
    },
    0x1c => reg32 lckr {
        // TODO: only available on GPIOs A & B for STM32F030x4 et al
        // TODO: only available on GPIOs A & B for STM32F0x1 et al.
        // TODO: only available on GPIOs A, B & D for STM32F303xB/C and STM32F358xC
        // TODO: only available on GPIOs A, B & D for STM32F37xx
        // TODO: only available on GPIOs A, B, C, D & F for STM32F303x6/8 and STM32F328x8
        // TODO: only available on GPIOs A, B & D for STM32F302xB/C
        // TODO: only available on GPIOs A, B, C, D & F for STM32F302x6/8
        0..15 => lck[16] : rw,
        16 => lckk : rw,
        17..31 => reserved : ro,
    },
    0x20 => reg32 afrl {
        // TODO: only available on GPIOs A & B for STM32F030x4 et al. (maybe?  manual not self-consistent)
        // TODO: values >= 0b1000 only allowed on ports A & B for STM32F334xx
        // TODO: values >= 0b1000 only allowed on ports A, B & D for STM32F37xx
        // TODO: values >= 0b1000 reserved on STM32F030x4 et al.
        // TODO: values >= 0b1000 reserved on STM32F0x1 et al.
        // TODO: values >= 0b1000 reserved on STM32L0x3
        // TODO: values >= 0b1000 reserved on STM32L0x2
        // TODO: values >= 0b1000 reserved on STM32L0x1
        0..31 => afr[8] : rw,
    },
    0x24 => reg32 afrh {
        // same caveats as afrl, except:
        // STM32F75xxx does not have this register for PORT J
        0..31 => afr[8] : rw,
    },
    0x28 => reg32 brr {
        // TODO: not available on STM32F4
        // TODO: not available on STM32F411xC/E
        // TODO: not available on STM32F205xx et al.
        // TODO: not available on STM32F401xB/C et al.
        // TODO: not available on STM32F75xxx et al.
        // TODO: not available on STM32F446xx
        0..15 => br[16] : wo,
        16..31 => reserved : ro,
    },
    0x2c => reg32 ascr {
        // TODO: only on STM32L4x6
        0..15 => asc[16] : rw,
        16..31 => reserved : ro,
    }
});
