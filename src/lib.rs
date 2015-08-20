#![feature(no_std)]
#![feature(plugin)]
#![no_std]
#![plugin(ioreg)]

#[macro_use] #[no_link]
extern crate ioreg;
extern crate volatile_cell;

pub mod stm32f0x2;
pub mod stm32f4;
pub mod stm32l476;

pub mod crc;
pub mod firewall;
pub mod flash;
pub mod gpio;
pub mod pwr;
pub mod timer;

mod init;
pub use init::init_memory;

pub mod vectors;
