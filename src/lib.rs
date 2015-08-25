#![feature(no_std)]
#![feature(plugin)]
#![no_std]
#![plugin(ioreg)]

#[macro_use] #[no_link]
extern crate ioreg;
extern crate volatile_cell;
extern crate arm_embedded_rt;

pub mod stm32f0x1;
pub mod stm32f4;
pub mod stm32l4x6;

pub mod crc;
pub mod gpio;
pub mod rtc;
pub mod timer;
