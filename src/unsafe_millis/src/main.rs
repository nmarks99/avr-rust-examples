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


use nano_hal::timer::millis;
use nano_hal::gpio::LED_BUILTIN;


#[no_mangle]
fn main() -> ! {
    
    const DELAY_TIME: u32 = 100;
     
    unsafe {
        LED_BUILTIN.set_output();
        LED_BUILTIN.low();
    }
    loop {
        let t0: u32 = millis();
        let mut tf: u32;
        let mut elap: u32;
        loop {
            tf = millis();
            elap = tf - t0;
            if elap >= DELAY_TIME {
                break
            }
        }
        unsafe { LED_BUILTIN.toggle(); }
    
    }
}






