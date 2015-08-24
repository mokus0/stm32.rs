VECTOR_BASE = 0x08000000;
NVIC_BASE   = 0x08000040;
NVIC_END    = 0x080000c0;

MEMORY {
    FLASH(RX):
        ORIGIN = 0x08000000,
        LENGTH = 128K
    
    SRAM(WXAIL):
        ORIGIN = 0x20000000,
        LENGTH = 16K
}

INCLUDE sections.ld;
INCLUDE thumbv6m_rt.ld;
INCLUDE stm32f0x1_peripherals.ld
