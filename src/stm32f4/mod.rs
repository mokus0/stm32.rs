// 
// Register interfaces for STM32F405/415, STM32F407/417,
// STM32F427/437 and STM32F429/439
// 
// See ST reference manual RM0090
// 

#![no_std]

pub mod rcc;
pub mod timer_opt;
pub mod usart;

use crc;
use gpio;
use rng;
use rtc;
use timer;

extern {
    #[link_name="stm32f4_CRC"]      pub static CRC:     crc::CRC;
    #[link_name="stm32f4_GPIOA"]    pub static GPIOA:   gpio::GPIO;
    #[link_name="stm32f4_GPIOB"]    pub static GPIOB:   gpio::GPIO;
    #[link_name="stm32f4_GPIOC"]    pub static GPIOC:   gpio::GPIO;
    #[link_name="stm32f4_GPIOD"]    pub static GPIOD:   gpio::GPIO;
    #[link_name="stm32f4_GPIOE"]    pub static GPIOE:   gpio::GPIO;
    #[link_name="stm32f4_GPIOF"]    pub static GPIOF:   gpio::GPIO;
    #[link_name="stm32f4_GPIOG"]    pub static GPIOG:   gpio::GPIO;
    #[link_name="stm32f4_GPIOH"]    pub static GPIOH:   gpio::GPIO;
    #[link_name="stm32f4_GPIOI"]    pub static GPIOI:   gpio::GPIO;
    #[link_name="stm32f4_GPIOJ"]    pub static GPIOJ:   gpio::GPIO;
    #[link_name="stm32f4_GPIOK"]    pub static GPIOK:   gpio::GPIO;
    #[link_name="stm32f4_RCC"]      pub static RCC:     rcc::RCC;
    #[link_name="stm32f4_RNG"]      pub static RNG:     rng::RNG;
    #[link_name="stm32f4_RTC"]      pub static RTC:     rtc::RTC;
    #[link_name="stm32f4_TIM2"]     pub static TIM2:    timer::GPTIM32;
    #[link_name="stm32f4_TIM2_OR"]  pub static TIM2_OR: timer_opt::TIM2_OPT;
    #[link_name="stm32f4_TIM3"]     pub static TIM3:    timer::GPTIM32;
    #[link_name="stm32f4_TIM4"]     pub static TIM4:    timer::GPTIM32;
    #[link_name="stm32f4_TIM5"]     pub static TIM5:    timer::GPTIM32;
    #[link_name="stm32f4_TIM5_OR"]  pub static TIM5_OR: timer_opt::TIM5_OPT;
    #[link_name="stm32f4_USART1"]   pub static USART1:  usart::USART;
    #[link_name="stm32f4_USART2"]   pub static USART2:  usart::USART;
    #[link_name="stm32f4_USART3"]   pub static USART3:  usart::USART;
    #[link_name="stm32f4_UART4"]    pub static UART4:   usart::USART;
    #[link_name="stm32f4_UART5"]    pub static UART5:   usart::USART;
    #[link_name="stm32f4_USART6"]   pub static USART6:  usart::USART;
    #[link_name="stm32f4_UART7"]    pub static UART7:   usart::USART;
    #[link_name="stm32f4_UART8"]    pub static UART8:   usart::USART;
}