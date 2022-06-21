#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use core::{panic::PanicInfo};
mod atmega328p; use atmega328p::*;
mod utils; use utils::*;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    // let dp = arduino_hal::Peripherals::take().unwrap();
    // let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    // let mut led = pins.d2.into_output();
    // led.set_low();

    unsafe {
        set_bit(DDRD, 2, true); // set as output

        loop {
            set_bit(PORTD, 2, true); // set high
            arduino_hal::delay_ms(500);            
            set_bit(PORTD, 2, false); // set low
            arduino_hal::delay_ms(500);
        }
    // }
    }
}