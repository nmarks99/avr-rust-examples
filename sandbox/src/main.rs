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
use nano_hal::timer::*;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe {
        
        timer1_init();
        let led = LED_BUILTIN;
        led.set_output(); 
        loop {

            led.high();

            while timer1_get_count() <= 20000 { }
            timer1_reset();

            led.low();

            while timer1_get_count() <= 20000 { }
            timer1_reset();
            
        }

        
    }
}
