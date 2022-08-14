use crate::atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
// use crate::meta::F_CPU;

pub const T1: Timer = Timer {pre: 64};
pub const MAX_TICKS: u32 = 65536;
pub const TICKS_PER_MS: u8 = 250;
pub static MILLIS: u32  = 0;


pub unsafe fn millis() -> u32 {
    MILLIS
}



// For now this is just timer1 which is a 16 bit timer
pub struct Timer {
    pub pre: u16
}

impl Timer {
   
    pub unsafe fn init(&self) {

        let pre:u8 = match &self.pre{ 
            1 => 1u8,
            8 => 2u8,
            64 => 3u8,
            256 => 4u8,
            1024 => 5u8,
            _ => loop {}
        };

        write_volatile(TCCR1B,pre); // set prescaler to 64 
    }

    pub unsafe fn get_count(&self) -> u16 {
        read_volatile(TCNT1)
    }

    pub unsafe fn reset(&self) {
        // let RESET_VAL: u16 = ( MAX_TICKS - (F_CPU/self.pre as u32)/1000 ) as u16;
        let RESET_VAL = 65535 - 250;
        write_volatile(TCNT1,RESET_VAL);
    }

    pub unsafe fn overflow_interrupt_enable(&self) {
        let current_bits = read_volatile(TIMSK1);
        let new_bits = current_bits | 0x01;
        write_volatile(TIMSK1, new_bits);
        avr_device::interrupt::enable();
    }

    pub unsafe fn overflow_flag(&self) -> bool {
        if (read_volatile(TIFR1) & (1 << *TOV1)) == 0 {
            false    
        }
        else {
            write_volatile(TIFR1, 1 << *TOV1);
            true 
        }
    }
}
