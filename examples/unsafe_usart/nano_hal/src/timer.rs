use crate::atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
use crate::meta::F_CPU;
use avr_device::interrupt::Mutex;
use core::cell::Cell;

pub const MAX_TICKS: u32 = 65535;

pub const T1: Timer = Timer {pre: 64};

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_OVF() {
    T1.reset(); // ensures we overflow every millisecond
    
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNT.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + 1);
    })
}


pub static MILLIS_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
pub fn millis() -> u32{
    // Initialize millis stuff the first time the function is called:
    let millis_count = avr_device::interrupt::free(|cs| MILLIS_COUNT.borrow(cs).get());
    if millis_count == 0 {
        unsafe { 
            T1.init();
            T1.overflow_interrupt_enable();
        }
    }
    millis_count
}


// For now this is just timer1 which is a 16 bit timer
pub struct Timer {
    pub pre: u16
}

impl Timer {
   
    pub unsafe fn init(&self) {

        let pre_out:u8 = match &self.pre{ 
            1 => 1u8,
            8 => 2u8,
            64 => 3u8,
            256 => 4u8,
            1024 => 5u8,
            _ => loop {}
        };

        write_volatile(TCCR1B,pre_out); // set prescaler
    }

    pub unsafe fn get_count(&self) -> u16 {
        read_volatile(TCNT1)
    }

    pub unsafe fn reset(&self) {
        // not sure why putting self.pre in the computation messes it up
        const PRESCALER: u32 = 64;
        let RESET_VAL: u16 = ( MAX_TICKS - (F_CPU/PRESCALER)/1000 ) as u16;
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












