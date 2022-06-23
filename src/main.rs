#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(non_snake_case)]

mod atmega328p;
mod utils; 
mod usart;

use utils::*;

// Define panic handler
use core::{panic::PanicInfo};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[arduino_hal::entry]
fn main() -> ! {

    unsafe {

        D2.set_output();
        D2.high();
        usart::usart_init(); 

        loop {
            let val: char = usart::read();
            if val == 'A' {
                D2.low();
                arduino_hal::delay_ms(500);
                D2.high();
                usart::print("Got an A\r\n");
            }
            else {
                usart::print("Got something else\r\n");                
            }
        }
    }
}
