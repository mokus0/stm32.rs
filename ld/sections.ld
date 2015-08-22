__flash_start   = ORIGIN(FLASH);
__flash_end     = ORIGIN(FLASH) + LENGTH(FLASH);

__sram_start    = ORIGIN(SRAM);
__sram_end      = ORIGIN(SRAM) + LENGTH(SRAM);

__data_start_flash = LOADADDR(.data);
__data_end_flash = LOADADDR(.data) + (__data_end_sram - __data_start_sram);

SECTIONS
{
    /*  */
    .text NVIC_END : {
        FILL(0xFF)
        __text_start = .;
        *(.text .text.*)
        __text_end = .;
        
        __rodata_start = .;
        SORT_BY_ALIGNMENT(*)(.rodata .rodata.*)
        __rodata_end = .;
    } > FLASH
    
    .data : ALIGN(0x04) {
        FILL(0xFF)
        __data_start_sram = .;

        SORT_BY_ALIGNMENT(*)(.data*)
        
        . = ALIGN(4);
        __data_end_sram = .;
    } > SRAM AT> FLASH
    
    .bss : ALIGN(0x04) {
        __bss_start = .;
        SORT_BY_ALIGNMENT(*)(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        __bss_end = .;
    } > SRAM
    
    .ARM.extab : {
        __extab_start = .;
        *(.ARM.extab* .gnu.linkonce.armextab.*)
        __extab_end = .;
    } > FLASH
    
    .ARM.exidx : {
        __exidx_start = .;
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
        __exidx_end = .;
    } > FLASH
    
    PROVIDE(__stack = __sram_end);
    
    .debug : {
        *(.debug_gdb_scripts)
    } > FLASH
}
INSERT AFTER .vect.nvic
