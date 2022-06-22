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

// general imports
mod atmega328p; use atmega328p::*;
mod utils; use utils::*;
mod usart;


#[arduino_hal::entry]
fn main() -> ! {
    unsafe {
        // let dp = arduino_hal::Peripherals::take().unwrap();
        // let pins = arduino_hal::pins!(dp);
        // let mut _serial = arduino_hal::default_serial!(dp,pins,9600);
        // arduino_hal::delay_ms(2000);
        // Digital pin 13 is also connected to an onboard LED marked "L"
        // let mut led = pins.d2.into_output();
        // led.set_low();
        
        
        set_bit(DDRD, 2, true); // set as output
        usart::usart_init();
        // set_bit(PORTD, 2, true); // set low

        let msg1: char = '2';
        let mut count:u8  = 0;
        loop {
            if count % 2 == 0 {
                set_bit(PORTD, 2, true);
            } else{
                set_bit(PORTD, 2, false);
            }

            usart::write_usart(msg1);
            arduino_hal::delay_ms(1000);
            if count == 254 {
                count = 0;
            }
            count = count + 1;


        }
    }
}
