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

        let msg: &str = "hello\r\n";
        let mut count:u32 = 0;
        loop {
            if count % 2 == 0 {
                D2.high();
            } else {D2.low()}
            usart::usart_print(msg);
            arduino_hal::delay_ms(1000);
            count += 1;

        }
    }
}
