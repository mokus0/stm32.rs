#![feature(no_std, plugin)]
#![no_std]
#![plugin(arm_rt_macro)]

extern crate stm32;
extern crate rlibc;

use stm32::stm32f4::*;

#[entry_point]
fn main() -> ! {
    // enable LED GPIOs 
    // (PD12 = green, PD13 = orange, PD14 = red, PD15 = blue)
    RCC.ahb1enr.set_gpiod_en(true);
    
    // set LED pins to "output"
    GPIOD.moder.set_mode(12, stm32::gpio::GPIO_moder_mode::Output)
               .set_mode(14, stm32::gpio::GPIO_moder_mode::Output);
    
    // enable TIM2
    RCC.apb1enr.set_tim2_en(true);
    
    // configure TIM2.  System clock at boot is 16 MHz and we haven't
    // changed it, so setting prescaler to 15999 will yield a 1 ms tick.
    // CEN bit switches the timer on.
    TIM2.psc.set_psc(15999);
    TIM2.cr1.set_cen(stm32::timer::GPTIM32_cr1_cen::Enable);
    
    // apply configuration changes to TIM2
    TIM2.egr.set_ug(true);
    
    loop {
        // Red on, Green off
        GPIOD.bsrr.set_br(12, true)
                  .set_bs(14, true);
        wait_ms(300);
        
        // Green on, Red off
        GPIOD.bsrr.set_bs(12, true)
                  .set_br(14, true);
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
