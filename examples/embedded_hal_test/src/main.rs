#![no_std]
#![feature(abi_avr_interrupt)]
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


// use embedded_hal as hal;
use avr_device::atmega328p;

// use hal::blocking::i2c;

// struct IMUDriver<I2C> {
//     i2c: I2C
// }

// impl i2c::Write<


#[no_mangle]
fn main() -> ! {
    let P = atmega328p::Peripherals::take().unwrap();
    P.PORTB.ddrb.write(|w| w.pb5().bit(true));
    P.PORTB.portb.write(|w| w.pb5().bit(true));
    
    loop {

    }
}




