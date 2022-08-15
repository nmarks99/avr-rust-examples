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
        // Sets PORTxn high
        set_bit(self.port,self.bit,true); 
    }

    pub unsafe fn low(&self) {
        // Sets PORTxn low
        set_bit(self.port,self.bit,false); 
    }

    pub unsafe fn set_output(&self) {
        // Sets DDRxn register high
        set_bit(self.ddr, self.bit, true);
    }

    pub unsafe fn set_input(&self) {
        // Sets DDRxn register low
        set_bit(self.ddr, self.bit, false);
        set_bit(self.port,self.bit,true); // enable pullup resistor
    }

    pub unsafe fn toggle(&self) {
        // Flips the value of PORTxn
        let current_bits = read_volatile(self.port);
        let new_bits = current_bits ^ _BV(self.bit);
        write_volatile(self.port, new_bits);
    }

    pub unsafe fn read(&self) -> bool {
        // Reads PORTxn and returns its value, true (logic 1) or false (logic 0)
        if (*self.port & (1 << self.bit)) == (1 << self.bit) {
            true
        } else {
            false
        }
    }

}


/* Digital pins */
pub const LED_BUILTIN: Pin = Pin { port: PORTB, bit: 5, ddr: DDRB};
pub const D2: Pin = Pin { port: PORTD, bit: 2, ddr: DDRD };
pub const D3: Pin = Pin { port: PORTD, bit: 3, ddr: DDRD };
pub const D4: Pin = Pin { port: PORTD, bit: 4, ddr: DDRD };
pub const D5: Pin = Pin { port: PORTD, bit: 5, ddr: DDRD };
pub const D6: Pin = Pin { port: PORTD, bit: 6, ddr: DDRD };
pub const D7: Pin = Pin { port: PORTD, bit: 7, ddr: DDRD };
pub const D8: Pin = Pin { port: PORTB, bit: 0, ddr: DDRB };
pub const D9: Pin = Pin { port: PORTB, bit: 1, ddr: DDRB };
pub const D10: Pin = Pin { port: PORTB, bit: 2, ddr: DDRB };
pub const D11: Pin = Pin { port: PORTB, bit: 3, ddr: DDRB };
pub const D12: Pin = Pin { port: PORTB, bit: 4, ddr: DDRB };
pub const D13: Pin = Pin { port: PORTB, bit: 5, ddr: DDRB };
