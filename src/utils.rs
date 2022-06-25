use core::ptr::write_volatile;
use atmega328p::*;


pub fn _BV(bit: u8) -> u8 {
    // Left shifts 1 the specfied amount
    1 << (bit)
}

pub unsafe fn set_bit(addr: *mut u8, bit: u8, state: bool) {
    // sets 'bit' at address 'addr' to 1 ('state' = true) or 0 ('state' = false)
    let val: u8;
    match state {
        true => val = *addr | _BV(bit),
        false => val = *addr & !_BV(bit)
    }
    write_volatile(addr,val);
}


/* GPIO pin definitions */
pub struct Pin {
    pub port: *mut u8,
    pub bit: u8,
    pub ddr: *mut u8
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

}

/* Digital pins */
pub const D2: Pin = Pin { port: PORTD, bit: 2, ddr: DDRD };
pub const D3: Pin = Pin { port: PORTD, bit: 3, ddr: DDRD };
pub const D4: Pin = Pin { port: PORTD, bit: 4, ddr: DDRD };
pub const D5: Pin = Pin { port: PORTD, bit: 5, ddr: DDRD };
pub const D6: Pin = Pin { port: PORTD, bit: 6, ddr: DDRD };
pub const D7: Pin = Pin { port: PORTD, bit: 7, ddr: DDRD };