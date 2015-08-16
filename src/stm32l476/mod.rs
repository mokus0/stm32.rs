#![no_std]

use firewall;
use flash;
use gpio;
use pwr;
use rcc;
use timer;

extern {
    pub static FIREWALL : firewall::FW;
    
    pub static FLASH : flash::FLASH;
    
    pub static GPIOA : gpio::GPIO;
    pub static GPIOB : gpio::GPIO;
    pub static GPIOC : gpio::GPIO;
    pub static GPIOD : gpio::GPIO;
    pub static GPIOE : gpio::GPIO;
    pub static GPIOF : gpio::GPIO;
    pub static GPIOG : gpio::GPIO;
    pub static GPIOH : gpio::GPIO;
    
    pub static PWR : pwr::PWR;
    
    pub static RCC : rcc::RCC;
    
    pub static TIM2 : timer::GPTIM32;
    pub static TIM3 : timer::GPTIM32;
    pub static TIM4 : timer::GPTIM32;
    pub static TIM5 : timer::GPTIM32;
}