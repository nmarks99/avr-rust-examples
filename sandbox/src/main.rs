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


use nano_hal::gpio::*;
use nano_hal::timer::*;

#[no_mangle]
fn main() -> ! {
    unsafe {
        
        LED_BUILTIN.set_output(); 
        LED_BUILTIN.high();

        loop {

        }
    }
}






