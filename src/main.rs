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




mod atmega328p;
mod utils; 
mod usart;
mod i2c;
use utils::*;

const BUFF_SIZE: usize = 50;

#[arduino_hal::entry]
fn main() -> ! {

    unsafe {

        D8.set_output();
        D8.high();
        usart::init(); 
        loop {
            let mut buff = [None;BUFF_SIZE];
            usart::readln(&mut buff);
            usart::println_recieved(&mut buff);
        }

    }
}
