#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(non_snake_case)]

use core::{panic::PanicInfo};
mod atmega328p; use atmega328p::*;
use core::intrinsics::volatile_store;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn LS1(bit: u8) -> u8 {
    // Left shifts 1 the specfied amount
    1 << (bit)
}

pub unsafe fn set_bit(addr: *mut u8, bit: u8, state: bool) {
    let val: u8;
    match state {
        true => val = *addr | LS1(bit),
        false => val = *addr & !LS1(bit)
    }
    volatile_store(addr,val);

}

// pub unsafe fn usart_init() {
//     set_bit(UCSR0A,0x2,false); // sets bit 1, the second bit (U2X0=0x2), to zero 
    
//     // Set to 8 bit data size
//     set_bit(UCSR0C,0x2,true); // UCSZ00 = 1
//     set_bit(UCSR0C,0x3,true); // UCSZ01 = 1
//     set_bit(UCSR0B,0x3,false); //UCSZ02 = 0

//     // Enable receiver and transitter
//     set_bit(UCSR0B,0x4,true); // Tx enable
//     set_bit(UCSR0B,0x5,true); // Rx enable

//     // Set baudrate
//     // let BAUD = ((8000000 / 115200) / 16) - 1;
//     // set_bit(UBRR0H,)


// }



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