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

#[no_mangle]
fn main() -> ! {
    unsafe {
        
        let led = LED_BUILTIN;
        led.set_output(); 
        led.high();

        loop {

        }
    }
}
