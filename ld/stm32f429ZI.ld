VECTOR_BASE = 0x08000000;
NVIC_BASE   = 0x08000040;
NVIC_END    = 0x08000188;

MEMORY {
    FLASH(RX):
        ORIGIN = 0x08000000,
        LENGTH = 2M
    
    SRAM1(WXAIL):
        ORIGIN = 0x20000000,
        LENGTH = 112K
    
    SRAM2(WXAIL):
        ORIGIN = 0x2001C000,
        LENGTH = 16K
    
    SRAM3(WXAIL):
        ORIGIN = 0x20020000,
        LENGTH = 64K
    
    CCM(WXAIL):
        ORIGIN = 0x10000000,
        LENGTH = 64K
}

__sram1_start   = ORIGIN(SRAM1);
__sram1_end     = ORIGIN(SRAM1) + LENGTH(SRAM1);

__sram2_start   = ORIGIN(SRAM2);
__sram2_end     = ORIGIN(SRAM2) + LENGTH(SRAM2);

__sram3_start   = ORIGIN(SRAM3);
__sram3_end     = ORIGIN(SRAM3) + LENGTH(SRAM3);

__ccm_start     = ORIGIN(CCM);
__ccm_end       = ORIGIN(CCM) + LENGTH(CCM);

REGION_ALIAS("SRAM", SRAM1);
INCLUDE sections.ld;

TIM2        = 0x40000000;
TIM2_OR     = 0x40000050;
TIM3        = 0x40000400;
TIM4        = 0x40000800;
TIM5        = 0x40000c00;
TIM5_OR     = 0x40000c50;

RCC         = 0x40023800;

GPIOA       = 0x40020000;
GPIOB       = 0x40020400;
GPIOC       = 0x40020800;
GPIOD       = 0x40020c00;
GPIOE       = 0x40021000;
GPIOF       = 0x40021400;
GPIOG       = 0x40021800;
GPIOH       = 0x40021c00;
GPIOI       = 0x40022000;
GPIOJ       = 0x40022400;
GPIOK       = 0x40022800;
