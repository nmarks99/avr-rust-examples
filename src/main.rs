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
        usart::usart_init(); 
        usart::println("Begin");
        loop {
            let mut buff = [None;50];
            usart::readln(&mut buff);
            usart::print_recieved(&mut buff);
            // for i in buff.iter() {
            //     match i {
            //         Some(c) => usart::send_byte(*c),
            //         None => break
            //     }
            // }
            // usart::send_byte('\n' as u8);
        }

    }
}
