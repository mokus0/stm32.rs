VECTOR_BASE = 0x08000000;
NVIC_BASE   = 0x08000040;
NVIC_END    = 0x08000188;

MEMORY {
    FLASH(RX):
        ORIGIN = 0x08000000,
        LENGTH = 1M
    
    SRAM1(WXAIL):
        ORIGIN = 0x20000000,
        LENGTH = 96K
    
    SRAM2(WXAIL):
        ORIGIN = 0x10000000,
        LENGTH = 32K
}

__sram1_start   = ORIGIN(SRAM1);
__sram1_end     = ORIGIN(SRAM1) + LENGTH(SRAM1);

__sram2_start   = ORIGIN(SRAM2);
__sram2_end     = ORIGIN(SRAM2) + LENGTH(SRAM2);

REGION_ALIAS("SRAM", SRAM1);
INCLUDE sections.ld;

SECTIONS
{
    .sram2 : {
        __sram2_start = .;
        *(.sram2 .sram2.*)
        __sram2_end = .;
    } > SRAM2
}
