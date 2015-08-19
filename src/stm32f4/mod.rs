#![no_std]

pub mod rcc;
pub mod timer_opt;

use gpio;
use timer;

extern {
    pub static GPIOA : gpio::GPIO;
    pub static GPIOB : gpio::GPIO;
    pub static GPIOC : gpio::GPIO;
    pub static GPIOD : gpio::GPIO;
    pub static GPIOE : gpio::GPIO;
    pub static GPIOF : gpio::GPIO;
    pub static GPIOG : gpio::GPIO;
    pub static GPIOH : gpio::GPIO;
    pub static GPIOI : gpio::GPIO;
    pub static GPIOJ : gpio::GPIO;
    pub static GPIOK : gpio::GPIO;
    
    pub static RCC : rcc::RCC;
    
    pub static TIM2 : timer::GPTIM32;
    pub static TIM2_OR : timer_opt::TIM2_OPT;
    pub static TIM3 : timer::GPTIM32;
    pub static TIM4 : timer::GPTIM32;
    pub static TIM5 : timer::GPTIM32;
    pub static TIM5_OR : timer_opt::TIM5_OPT;
}