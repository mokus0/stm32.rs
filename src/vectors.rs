#![no_std]

use core::option::Option;
use core::option::Option::{Some, None};

#[repr(C)]
pub struct ExceptionVectors {
    // can't declare initial_stack_ptr as a static u32 because it must be a 
    // pointer in order to be relocatable.  Also can't declare as const* ()
    // since 
    initial_stack_ptr:      unsafe extern fn(),
    reset_handler:          unsafe extern fn(),
    // TODO: most if not all of these should probably be required
    non_maskable_interrupt: Option<unsafe extern fn()>,
    hard_fault:             Option<unsafe extern fn()>,
    mem_manage:             Option<unsafe extern fn()>,
    bus_fault:              Option<unsafe extern fn()>,
    usage_fault:            Option<unsafe extern fn()>,
    reserved_a:             [u32; 4],
    service_call:           Option<unsafe extern fn()>,
    debug:                  Option<unsafe extern fn()>,
    reserved_b:             u32,
    pend_service_call:      Option<unsafe extern fn()>,
    sys_tick:               Option<unsafe extern fn()>,
}

// TODO: some way to generate vector entries on demand by
//       tagging arbitrary functions with some sort of annotation
// TODO: device-specific layout (Cortex-M0 vs M3/4, etc)
// TODO: similar treatment for NVIC
#[no_mangle] #[link_section=".exception_vectors"]
pub static EXCEPTION_VECTORS : ExceptionVectors = ExceptionVectors {
    initial_stack_ptr:      __stack,
    reset_handler:          main,
    non_maskable_interrupt: None,
    hard_fault:             None,
    mem_manage:             None,
    bus_fault:              None,
    usage_fault:            None,
    reserved_a:             [0, 0, 0, 0],
    service_call:           None,
    debug:                  None,
    reserved_b:             0,
    pend_service_call:      None,
    sys_tick:               None,
};

extern {
    fn __stack();
    fn main();
}

