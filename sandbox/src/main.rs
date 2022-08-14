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


use nano_hal::timer::T1;
// use nano_hal::timer::millis;
use nano_hal::gpio::LED_BUILTIN;

static mut MILLIS: u32 = 0;

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_OVF() {
    // T1.reset();    
    MILLIS += 1;
    LED_BUILTIN.toggle()
}



#[no_mangle]
fn main() -> ! {
    unsafe {
        LED_BUILTIN.set_output();
        LED_BUILTIN.low();
        T1.init(); 
        T1.overflow_interrupt_enable();

        loop {
        }
    } 
}






