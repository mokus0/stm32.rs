/* 
 * Peripheral map for STM32F405/415, STM32F407/417,
 * STM32F427/437 and STM32F429/439
 * 
 * See ST reference manual RM0090
 */

MEMORY {
    BKPSRAM(WXAIL):
        ORIGIN = 0x40024000,
        LENGTH = 4K
}

SECTIONS {
    .bkpsram : {
        __bkpsram_start = .;
        *(.bkpsram .bkpsram.*)
        __bkpsram_end = .;
    } > BKPSRAM
}

stm32f4_CRC         = 0x40023000;
stm32f4_GPIOA       = 0x40020000;
stm32f4_GPIOB       = 0x40020400;
stm32f4_GPIOC       = 0x40020800;
stm32f4_GPIOD       = 0x40020c00;
stm32f4_GPIOE       = 0x40021000;
stm32f4_GPIOF       = 0x40021400;
stm32f4_GPIOG       = 0x40021800;
stm32f4_GPIOH       = 0x40021c00;
stm32f4_GPIOI       = 0x40022000;
stm32f4_GPIOJ       = 0x40022400;
stm32f4_GPIOK       = 0x40022800;
stm32f4_RCC         = 0x40023800;
stm32f4_RNG         = 0x50060800;
stm32f4_RTC         = 0x40002800;
stm32f4_TIM2        = 0x40000000;
stm32f4_TIM2_OR     = 0x40000050;
stm32f4_TIM3        = 0x40000400;
stm32f4_TIM4        = 0x40000800;
stm32f4_TIM5        = 0x40000c00;
stm32f4_TIM5_OR     = 0x40000c50;
stm32f4_USART1      = 0x40011000;
stm32f4_USART2      = 0x40004400;
stm32f4_USART3      = 0x40004800;
stm32f4_UART4       = 0x40004C00;
stm32f4_UART5       = 0x40005000;
stm32f4_USART6      = 0x40011400;
stm32f4_UART7       = 0x40007800;
stm32f4_UART8       = 0x40007C00;
