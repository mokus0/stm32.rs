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

TIM2        = 0x40000000;
TIM3        = 0x40000400;
TIM4        = 0x40000800;
TIM5        = 0x40000c00;
TIM6        = 0x40001000;
TIM7        = 0x40001400;

PWR         = 0x40007000;

FIREWALL    = 0x40011C00;

RCC         = 0x40021000;

FLASH       = 0x40022000;

GPIOA       = 0x48000000;
GPIOB       = 0x48000400;
GPIOC       = 0x48000800;
GPIOD       = 0x48000c00;
GPIOE       = 0x48001000;
GPIOF       = 0x48001400;
GPIOG       = 0x48001800;
GPIOH       = 0x48001c00;

