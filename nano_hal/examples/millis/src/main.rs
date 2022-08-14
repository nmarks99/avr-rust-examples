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
use nano_hal::gpio::LED_BUILTIN;
use core::cell::Cell;
use avr_device::interrupt::Mutex;

// static mut MILLIS: u32 = 0;
static MILLIS_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));


#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_OVF() {
    T1.reset(); // ensures we overflow every millisecond
    
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNT.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + 1);
    })
}


fn millis() -> u32{
    avr_device::interrupt::free(|cs| MILLIS_COUNT.borrow(cs).get())
}


#[no_mangle]
fn main() -> ! {
    
    unsafe {
    LED_BUILTIN.set_output();
    LED_BUILTIN.low();
    T1.init(); 
    T1.overflow_interrupt_enable();
    }

    loop {
        let t0: u32 = millis();
        let mut tf: u32;
        let mut elap: u32;
        loop {
            tf = millis();
            elap = tf - t0;
            if elap >= 1000 {
                break
            }
        }
        unsafe { LED_BUILTIN.toggle(); }
    
    }
}






