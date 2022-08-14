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

// const RESET_VAL:u16 = (65535 - (meta::F_CPU/64)/1000) as u16;
// const TICKS_PER_MS:u8 = 250;
static mut MILLIS: u32 = 0;


#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_OVF() {
    T1.reset();    
    MILLIS += 1;
}



#[no_mangle]
fn main() -> ! {
    unsafe {
        LED_BUILTIN.set_output();
        T1.init(); 
        T1.overflow_interrupt_enable();
        let mut c:u32 = 0; 
        loop{
            let t0: u32 = MILLIS;
            
            loop {
                let tf:u32 = MILLIS;
                if (tf - t0) >= 2000 {
                    break;
                }
            }
            if c % 2 == 0 {
                LED_BUILTIN.high();
            }
            else {
                LED_BUILTIN.low();
            }
            c += 1;
        }
    } 
}






