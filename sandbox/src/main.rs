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
use nano_hal::delay;

#[arduino_hal::entry]
fn main() -> ! {
    unsafe {
        
        let led = D3_OUTPUT;
        led.set_output(); 
        loop {
            
            led.high();
            delay!(1000.0);
            led.low();
            delay!(1000.0);
            
        }
    }
}
