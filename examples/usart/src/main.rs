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
use heapless::String;
use core::fmt::Write;

#[no_mangle]
fn main() -> ! {
    unsafe {
        usart::init();
        let mut m: String<50> = String::from("");
        let x: u8 = 2;
        write!(&mut m, "howdy {}",x).unwrap();
        usart::println("hi");
        let mm: &str = m.as_str();
        usart::println(mm); 

        loop {

        }

    }
}






