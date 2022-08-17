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



use nano_hal::usart;
use nano_hal::adc;

#[no_mangle]
fn main() -> ! {
    unsafe {
        adc::init();
        usart::init();
        usart::println("hello");

        loop {
            let val = adc::read(adc::ADC0);
            let val: u8 = (9)*(val as u8);
            usart::send_byte(val);
            usart::send_byte('\n' as u8);
        }

    }
}






