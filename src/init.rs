#![no_std]

/// Load .data section from FLASH to SRAM and initialize .bss section
#[inline(always)]
pub fn init_memory() {
    // Let's get this party started!
    
    // Copy .data section from FLASH to SRAM
    unsafe {
        let mut src = &    __data_start_flash as *const u32;
        let     end = &    __data_end_flash   as *const u32;
        let mut dst = &mut __data_start_sram  as *mut   u32;
        
        while src < end
        {
            *dst = *src;
            src = ((src as usize) + 4) as *const u32;
            dst = ((dst as usize) + 4) as *mut   u32;
        }
    }
    // Initialize .bss section in SRAM
    unsafe {
        let mut dst = &mut __bss_start as *mut   u32;
        let     end = &    __bss_end   as *const u32;
        
        while (dst as *const u32) < end
        {
            *dst = 0;
            dst = ((dst as usize) + 4) as *mut   u32;
        }
    }
}

// these symbols are defined in the linker script, and describe the location
// of various sections of memory.  They are declared as concrete types (u32)
// but their contents are meaningless (and completely unsafe to touch) - it
// is their addresses we need.
//
// we indend to copy/zero these sections one full word at a time, so these
// must all be word-aligned by the linker script.
extern {
    /// The first word in the .data section in SRAM
    #[no_mangle]
    static mut __data_start_sram : u32;

    /// The first word in the .data section in FLASH
    #[no_mangle]
    static __data_start_flash : u32;

    /// The first word *after* the .data section in FLASH
    #[no_mangle]
    static __data_end_flash : u32;

    /// The first word in the .bss section in SRAM
    #[no_mangle]
    static mut __bss_start : u32;

    /// The first word *after* the .bss section in SRAM
    #[no_mangle]
    static __bss_end : u32;
}