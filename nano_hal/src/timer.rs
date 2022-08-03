use crate::atmega328p::*;
use core::panicking::panic;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

pub unsafe fn timer1_init() {
    write_volatile(TCCR1B,0b00000011); // set prescaler to 64 
}

pub unsafe fn timer1_get_count() -> u16 {
    read_volatile(TCNT1)
}

pub unsafe fn timer1_reset() {
    write_volatile(TCNT1,0u16);
}

// pub const fn get_clock_select(pre:u16) -> [u8;3] {
   //
    // let CSB: [u8;3] = match pre {
        //
        // 1 => [0,0,1],
        // 8 => [0,1,0],
        // 64 => [0,1,1],
        // 256 => [1,0,0],
        // 1024 => [1,0,1],
        // _ => panic!("Invalid prescaler value")
    // };
    // CSB
// }


