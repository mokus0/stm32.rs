/// 
/// Register interfaces for STM32F0x1, STM32F0x2 and STM32F0x8
/// 
/// See ST reference manual RM0091
/// 

#![no_std]

pub mod rcc;

use gpio;
use timer;

extern {
    #[link_name="stm32f0x1_GPIOA"]  pub static GPIOA:   gpio::GPIO;
    #[link_name="stm32f0x1_GPIOB"]  pub static GPIOB:   gpio::GPIO;
    #[link_name="stm32f0x1_GPIOC"]  pub static GPIOC:   gpio::GPIO;
    #[link_name="stm32f0x1_GPIOD"]  pub static GPIOD:   gpio::GPIO;
    #[link_name="stm32f0x1_GPIOE"]  pub static GPIOE:   gpio::GPIO;
    #[link_name="stm32f0x1_GPIOF"]  pub static GPIOF:   gpio::GPIO;
    
    #[link_name="stm32f0x1_RCC"]    pub static RCC:     rcc::RCC;
    
    #[link_name="stm32f0x1_TIM2"]   pub static TIM2:    timer::GPTIM32;
    #[link_name="stm32f0x1_TIM3"]   pub static TIM3:    timer::GPTIM32;
}