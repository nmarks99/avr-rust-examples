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
            // let rx_msg: char = usart::read();
            // let mut a = [0; 2];
            // let m = rx_msg.encode_utf8(&mut a);
            // usart::print(m);
            usart::print("Howdy\r\n");
            arduino_hal::delay_ms(1000);
        }
    }
}
