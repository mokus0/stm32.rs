VECTOR_BASE = 0x08000000;
NVIC_BASE   = 0x08000040;
NVIC_END    = 0x08000188;

MEMORY {
    FLASH(RX):
        ORIGIN = 0x08000000,
        LENGTH = 1M
    
    SRAM1(WXAIL):
        ORIGIN = 0x20000000,
        LENGTH = 112K
    
    SRAM2(WXAIL):
        ORIGIN = 0x2001C000,
        LENGTH = 16K
    
    CCM(WXAIL):
        ORIGIN = 0x10000000,
        LENGTH = 64K
}

__sram1_start   = ORIGIN(SRAM1);
__sram1_end     = ORIGIN(SRAM1) + LENGTH(SRAM1);

__sram2_start   = ORIGIN(SRAM2);
__sram2_end     = ORIGIN(SRAM2) + LENGTH(SRAM2);

__ccm_start     = ORIGIN(CCM);
__ccm_end       = ORIGIN(CCM) + LENGTH(CCM);

REGION_ALIAS("SRAM", SRAM1);

INCLUDE sections.ld;
INCLUDE thumbv7em_rt.ld;
INCLUDE stm32f4_peripherals.ld;
