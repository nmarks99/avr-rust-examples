#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(non_snake_case)]

// Define panic handler
use core::{panic::PanicInfo};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




use nano_hal::gpio::LED_BUILTIN;
// use nano_hal::gpio::*;
use nano_halo::timer::*;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe {
        
        timer1_init();
        let led = LED_BUILTIN;
        LED_BUILTIN.low();
        loop {
            
            LED_BUILTIN.low();
            
            while timer1_millis() <= 12500 { }
            timer1_reset();
           
            LED_BUILTIN.high(); 
          
            while timer1_millis() <= 12500 { }
            timer1_reset();
            
        }

        
    }
}
