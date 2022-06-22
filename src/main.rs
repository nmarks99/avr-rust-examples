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

// general imports
mod atmega328p; use atmega328p::*;
mod utils; use utils::*;
mod usart;


#[arduino_hal::entry]
fn main() -> ! {
    unsafe {
        
        set_bit(DDRD, 2, true); // set as output
        usart::usart_init();
        // set_bit(PORTD, 2, true); // set low
        let msg1: &str = "hello";
        loop {
            usart::usart_println(msg1);
            arduino_hal::delay_ms(1000);
        }
    }
}
