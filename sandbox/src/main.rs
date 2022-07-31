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

#[arduino_hal::entry]
fn main() -> ! {
    unsafe {

        let button = D12_INPUT;
        let led = D3_OUTPUT;
        loop {
            if button.read() == true {
                led.high();
            }
            else {
                led.low();
            }
        }
    }
}
