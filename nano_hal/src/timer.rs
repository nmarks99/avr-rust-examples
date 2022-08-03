use crate::atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

pub unsafe fn timer1_init() {
    write_volatile(TCCR1B,0b00000011); // set prescaler to 64 
}

pub unsafe fn timer1_millis() -> u16 {
    read_volatile(TCNT1)
}

pub unsafe fn timer1_reset() {
    write_volatile(TCNT1,0u16);
}




// pub struct Timer {
//     t: u8, // 0, 1, or 2
//     pre: u16
// }

// impl Timer {

//     unsafe fn init() {
//         write_volatile(TCCR1B,)
//     }
// }
