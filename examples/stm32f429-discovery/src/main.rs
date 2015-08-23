#![feature(no_std, plugin)]
#![no_std]
#![plugin(arm_rt_macro)]

extern crate stm32;
extern crate rlibc;

use stm32::stm32f4::*;

#[entry_point]
fn main() -> ! {
    // enable LED GPIOs (PG13 = green, PG14 = red)
    RCC.ahb1enr.set_gpiog_en(true);
    
    // set LED pins to "output"
    GPIOG.moder.set_mode(13, stm32::gpio::GPIO_moder_mode::Output)
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
        GPIOG.bsrr.set_br(13, true)
                  .set_bs(14, true);
        wait_ms(300);
        
        // Green on, Red off
        GPIOG.bsrr.set_bs(13, true)
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
