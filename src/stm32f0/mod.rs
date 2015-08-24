#![no_std]

pub mod rcc;

use gpio;
use timer;

extern {
    pub static GPIOA : gpio::GPIO;
    pub static GPIOB : gpio::GPIO;
    pub static GPIOC : gpio::GPIO;
    pub static GPIOD : gpio::GPIO;
    pub static GPIOE : gpio::GPIO;
    pub static GPIOF : gpio::GPIO;
    
    pub static RCC : rcc::RCC;
    
    pub static TIM2 : timer::GPTIM32;
    pub static TIM3 : timer::GPTIM32;
}