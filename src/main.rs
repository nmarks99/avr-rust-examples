#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(non_snake_case)]

mod atmega328p;
mod utils; 
mod usart;
mod i2c;
mod gpio;

// Define panic handler
use core::{panic::PanicInfo};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}



use gpio::*;

const BUFF_SIZE: usize = 50;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe {

        D2.set_input();
        D3.set_output();
        loop {
            
            if D2.read() == true {
                D3.high();
            }
            else {
                D3.low();
            }
            // arduino_hal::delay_ms(50);
        }




    }
}
