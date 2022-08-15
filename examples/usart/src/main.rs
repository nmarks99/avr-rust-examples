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
use itoa;
use heapless::String;
use core::fmt::Write;

#[no_mangle]
fn main() -> ! {
    unsafe {
        usart::init();
        usart::println("asdf");
        
        let mut b = itoa::Buffer::new();
        let x: u32 = 163832;
        let num_buff = b.format(x);

        
        let mut s: String<50> = String::from("b");
        write!(&mut s, "Hey this is a string {}", num_buff).unwrap();
        usart::println(&s[..]); 

        loop {

        }

    }
}






