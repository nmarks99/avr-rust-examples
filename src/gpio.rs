use atmega328p::*;
use utils::*;

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

    // pub unsafe fn set_output(&self) {
    //     set_bit(self.ddr, self.bit, true)
    // }

    // pub unsafe fn set_input(&self) {
    //     set_bit(self.ddr, self.bit, false)
    // }

    pub unsafe fn read(&self) -> bool {
        if (*self.port & (1 << self.bit)) == (1 << self.bit) {
            true
        } else {
            false
        }
    }

}

/* Digital pins */
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
pub const D12: Pin = Pin { port: PINB, bit: 4, ddr: DDRB };
pub const D13: Pin = Pin { port: PORTB, bit: 5, ddr: DDRB };

// /* Analog pins */
// pub const A0: Pin = Pin { port: PORTC, bit: 0, ddr: DDRC };
// pub const A1: Pin = Pin { port: PORTC, bit: 1, ddr: DDRC };
// pub const A2: Pin = Pin { port: PORTC, bit: 2, ddr: DDRC };
// pub const A3: Pin = Pin { port: PORTC, bit: 3, ddr: DDRC };
// pub const A4: Pin = Pin { port: PORTC, bit: 4, ddr: DDRC };
// pub const A5: Pin = Pin { port: PORTC, bit: 5, ddr: DDRC };