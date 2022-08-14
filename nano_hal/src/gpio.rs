use crate::atmega328p::*;
use crate::meta::*;
use core::ptr::{read_volatile, write_volatile};

/* GPIO pin definitions */
pub struct Pin {
    pub port: *mut u8,
    pub bit: u8,
    pub ddr: *mut u8,
}

impl Pin {


    pub unsafe fn high(&self) {
       set_bit(self.port,self.bit,true); 
    }

    pub unsafe fn low(&self) {
        set_bit(self.port,self.bit,false); 
    }

    pub unsafe fn set_output(&self) {
        set_bit(self.ddr, self.bit, true)
    }

    pub unsafe fn set_input(&self) {
        set_bit(self.ddr, self.bit, false)
    }

    pub unsafe fn toggle(&self) {
        let current_bits = read_volatile(self.port);
        let new_bits = current_bits ^ _BV(self.bit);
        write_volatile(self.port, new_bits);
    }

    pub unsafe fn read(&self) -> bool {
        if (*self.port & (1 << self.bit)) == (1 << self.bit) {
            true
        } else {
            false
        }
    }

}

pub const LED_BUILTIN: Pin = Pin { port: PORTB, bit: 5, ddr: DDRB};

/* Digital pins */
pub const D2_INPUT: Pin = Pin { port: PIND, bit: 2, ddr: DDRD };
pub const D2_OUTPUT: Pin = Pin { port: PORTD, bit: 2, ddr: DDRD };

pub const D3_INPUT: Pin = Pin { port: PIND, bit: 3, ddr: DDRD };
pub const D3_OUTPUT: Pin = Pin { port: PORTD, bit: 3, ddr: DDRD };

pub const D4_INPUT: Pin = Pin { port: PIND, bit: 4, ddr: DDRD };
pub const D4_OUTPUT: Pin = Pin { port: PORTD, bit: 4, ddr: DDRD };

pub const D5_INPUT: Pin = Pin { port: PIND, bit: 5, ddr: DDRD };
pub const D5_OUTPUT: Pin = Pin { port: PORTD, bit: 5, ddr: DDRD };

pub const D6_INPUT: Pin = Pin { port: PIND, bit: 6, ddr: DDRD };
pub const D6_OUTPUT: Pin = Pin { port: PORTD, bit: 6, ddr: DDRD };

pub const D7_INPUT: Pin = Pin { port: PIND, bit: 7, ddr: DDRD };
pub const D7_OUTPUT: Pin = Pin { port: PORTD, bit: 7, ddr: DDRD };

pub const D8_INPUT: Pin = Pin { port: PINB, bit: 0, ddr: DDRB };
pub const D8_OUTPUT: Pin = Pin { port: PORTB, bit: 0, ddr: DDRB };

pub const D9_INPUT: Pin = Pin { port: PINB, bit: 1, ddr: DDRB };
pub const D9_OUTPUT: Pin = Pin { port: PORTB, bit: 1, ddr: DDRB };

pub const D10_INPUT: Pin = Pin { port: PINB, bit: 2, ddr: DDRB };
pub const D10_OUTPUT: Pin = Pin { port: PORTB, bit: 2, ddr: DDRB };

pub const D11_INPUT: Pin = Pin { port: PINB, bit: 3, ddr: DDRB };
pub const D11_OUTPUT: Pin = Pin { port: PORTB, bit: 3, ddr: DDRB };

pub const D12_OUTPUT: Pin = Pin { port: PORTB, bit: 4, ddr: DDRB };
pub const D12_INPUT: Pin = Pin { port: PINB, bit: 4, ddr: DDRB };

pub const D13_INPUT: Pin = Pin { port: PINB, bit: 5, ddr: DDRB };
pub const D13_OUTPUT: Pin = Pin { port: PORTB, bit: 5, ddr: DDRB };



// /* Analog pins */
// pub const A0: Pin = Pin { port: PORTC, bit: 0, ddr: DDRC };
// pub const A1: Pin = Pin { port: PORTC, bit: 1, ddr: DDRC };
// pub const A2: Pin = Pin { port: PORTC, bit: 2, ddr: DDRC };
// pub const A3: Pin = Pin { port: PORTC, bit: 3, ddr: DDRC };
// pub const A4: Pin = Pin { port: PORTC, bit: 4, ddr: DDRC };
// pub const A5: Pin = Pin { port: PORTC, bit: 5, ddr: DDRC };
