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


// use nano_hal::gpio::*;
use avr_device::atmega328p;

#[no_mangle]
fn main() -> ! {
    
    let p = atmega328p::Peripherals::take().unwrap();
    p.PORTB.ddrb.write(|w| unsafe{w.bits(0xFF)}); // set as output
    p.PORTB.portb.write(|w| unsafe{w.bits(0x0)}); 

    loop {

    }
}






