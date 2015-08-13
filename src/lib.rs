#![feature(no_std)]
#![feature(plugin)]
#![no_std]
#![plugin(ioreg)]

#[macro_use] #[no_link]
extern crate ioreg;
extern crate volatile_cell;

pub mod crc;
pub mod firewall;
pub mod flash;
pub mod gpio;
pub mod pwr;
pub mod rcc;
