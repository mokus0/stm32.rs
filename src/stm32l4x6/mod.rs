// 
// Register interfaces for STM32L4x6
// 
// See ST reference manual RM0351
// 

#![no_std]

pub mod firewall;
pub mod flash;
pub mod pwr;
pub mod rcc;
pub mod timer_opt;

use gpio;
use timer;

extern {
    #[link_name="stm32l4x6_FIREWALL"]   pub static FIREWALL:    firewall::FW;
    
    #[link_name="stm32l4x6_FLASH"]      pub static FLASH:       flash::FLASH;
    
    #[link_name="stm32l4x6_GPIOA"]      pub static GPIOA:       gpio::GPIO;
    #[link_name="stm32l4x6_GPIOB"]      pub static GPIOB:       gpio::GPIO;
    #[link_name="stm32l4x6_GPIOC"]      pub static GPIOC:       gpio::GPIO;
    #[link_name="stm32l4x6_GPIOD"]      pub static GPIOD:       gpio::GPIO;
    #[link_name="stm32l4x6_GPIOE"]      pub static GPIOE:       gpio::GPIO;
    #[link_name="stm32l4x6_GPIOF"]      pub static GPIOF:       gpio::GPIO;
    #[link_name="stm32l4x6_GPIOG"]      pub static GPIOG:       gpio::GPIO;
    #[link_name="stm32l4x6_GPIOH"]      pub static GPIOH:       gpio::GPIO;
    
    #[link_name="stm32l4x6_PWR"]        pub static PWR:         pwr::PWR;
    
    #[link_name="stm32l4x6_RCC"]        pub static RCC:         rcc::RCC;
    
    #[link_name="stm32l4x6_TIM2"]       pub static TIM2:        timer::GPTIM32;
    #[link_name="stm32l4x6_TIM2_OR"]    pub static TIM2_OR:     timer_opt::TIM2_OPT;
    #[link_name="stm32l4x6_TIM3"]       pub static TIM3:        timer::GPTIM32;
    #[link_name="stm32l4x6_TIM3_OR"]    pub static TIM3_OR:     timer_opt::TIM3_OPT;
    #[link_name="stm32l4x6_TIM4"]       pub static TIM4:        timer::GPTIM32;
    #[link_name="stm32l4x6_TIM5"]       pub static TIM5:        timer::GPTIM32;
}