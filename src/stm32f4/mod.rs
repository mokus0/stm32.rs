#![no_std]

pub mod rcc;
pub mod timer_opt;

use gpio;
use timer;
use usart;

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
    
    pub static USART1 : usart::USART;
    pub static USART2 : usart::USART;
    pub static USART3 : usart::USART;
    pub static UART4 : usart::USART;
    pub static UART5 : usart::USART;
    pub static USART6 : usart::USART;
    pub static UART7 : usart::USART;
    pub static UART8 : usart::USART;
}