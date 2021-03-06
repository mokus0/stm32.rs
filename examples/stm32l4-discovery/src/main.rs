#![feature(no_std, plugin)]
#![no_std]
#![plugin(arm_rt_macro)]

extern crate stm32;
extern crate rlibc;

use stm32::stm32l4x6::*;

#[entry_point]
fn main() -> ! {
    // enable LED GPIOs (PB2 = red, PE8 = green)
    RCC.ahb2enr.set_gpiob_en(true).set_gpioe_en(true);
    
    // set LED pins to "output"
    GPIOB.moder.set_mode(2, stm32::gpio::GPIO_moder_mode::Output);
    GPIOE.moder.set_mode(8, stm32::gpio::GPIO_moder_mode::Output);
    
    // enable TIM2
    RCC.apb1enr1.set_tim2_en(true);
    
    // configure TIM2.  System clock at boot is 4 MHz and we haven't
    // changed it, so setting prescaler to 3999 will yield a 1 ms tick.
    // CEN bit switches the timer on.
    TIM2.psc.set_psc(3999);
    TIM2.cr1.set_cen(stm32::timer::GPTIM32_cr1_cen::Enable);
    
    // apply configuration changes to TIM2
    TIM2.egr.set_ug(true);
    
    loop {
        // Red on, Green off
        GPIOB.bsrr.set_bs(2, true);
        GPIOE.bsrr.set_br(8, true);
        wait_ms(300);
        
        // Green on, Red off
        GPIOB.bsrr.set_br(2, true);
        GPIOE.bsrr.set_bs(8, true);
        wait_ms(300);
    }
}

fn wait_ms(ms : u32) {
    let start = TIM2.cnt.cnt();
    while TIM2.cnt.cnt() - start < ms
    {
        // just spin
    }
}
