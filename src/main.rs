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


const BUFF_SIZE: usize = 50;

#[arduino_hal::entry]
fn main() -> ! {

    unsafe {

        D7.set_output();
        D7.high();
        usart::usart_init(); 
        loop {
            // let mut buff = [None;BUFF_SIZE];
            // usart::readln(&mut buff);
            // usart::println_recieved(&mut buff);
        }

    }
}
