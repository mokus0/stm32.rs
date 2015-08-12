#![feature(no_std)]
#![feature(plugin)]
#![no_std]
#![plugin(ioreg)]

#[macro_use] #[no_link]
extern crate ioreg;
extern crate volatile_cell;

pub mod gpio;