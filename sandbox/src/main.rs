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
use nano_hal::timer::*;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe {
        
        timer1_init();
        LED_BUILTIN.set_output();
        LED_BUILTIN.high();
        loop {

            LED_BUILTIN.high();

            while timer1_millis() <= 3000 { }
            timer1_reset();

            LED_BUILTIN.low();

            while timer1_millis() <= 3000 { }
            timer1_reset();
            
        }

        
    }
}
