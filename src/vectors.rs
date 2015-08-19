#![no_std]

use core::option::Option;
use core::option::Option::{Some, None};

/// per the ARMv7-M Architecture Reference Manual
/// Section B1.5.2 (Table B1-4)
/// 
/// Applies to all Cortex-M3, -M3 and -M7 microcontrollers.
#[repr(C)]
pub struct ExceptionVectors {
    // can't declare initial_stack_ptr as a static u32 because it must be a 
    // pointer in order to be relocatable.  Also can't declare as const* ()
    // since 
    initial_stack_ptr:      unsafe extern fn(),
    reset:                  unsafe extern fn(),
    // TODO: most if not all of these should probably be required
    non_maskable_interrupt: Option<unsafe extern fn()>,
    hard_fault:             Option<unsafe extern fn()>,
    mem_manage:             Option<unsafe extern fn()>, // not in ARMv6-M
    bus_fault:              Option<unsafe extern fn()>, // not in ARMv6-M
    usage_fault:            Option<unsafe extern fn()>, // not in ARMv6-M
    reserved7:              u32,
    reserved8:              u32,
    reserved9:              u32,
    reserved10:             u32,
    sv_call:                Option<unsafe extern fn()>,
    debug_monitor:          Option<unsafe extern fn()>, // not in ARMv6-M
    reserved13:             u32,
    pend_sv:                Option<unsafe extern fn()>,
    sys_tick:               Option<unsafe extern fn()>,
}

// TODO: some way to generate vector entries on demand by
//       tagging arbitrary functions with some sort of annotation
// TODO: device-specific layout (Cortex-M0 vs M3/4, etc)
// TODO: similar treatment for NVIC
#[no_mangle] #[link_section=".exception_vectors"]
pub static EXCEPTION_VECTORS : ExceptionVectors = ExceptionVectors {
    initial_stack_ptr:      __stack,
    reset:                  main,
    non_maskable_interrupt: None,
    hard_fault:             None,
    mem_manage:             None,
    bus_fault:              None,
    usage_fault:            None,
    reserved7:              0,
    reserved8:              0,
    reserved9:              0,
    reserved10:             0,
    sv_call:                None,
    debug_monitor:          None,
    reserved13:             0,
    pend_sv:                None,
    sys_tick:               None,
};

extern {
    fn __stack();
    fn main();
}

