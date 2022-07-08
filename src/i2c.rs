// https://embedds.com/programming-avr-i2c-interface/

use utils::*;
use atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

// pub const SCL_PRE: u8 = 1;
// pub const SCL_FREQ: u32 = F_CPU / 16 + 2*(TWBR) * (SCL_PRE)

pub unsafe fn init() {
    // Set SCL to 400kHz
    write_volatile(TWSR, 0x00);
    write_volatile(TWBR, 0x0C); // 0x0C = 12, 16000000/(16+(2*12)) = 400 kHz
    
    // Enable TWI
    set_bit(TWCR, *TWEN, true)
}

pub unsafe fn start() {
    set_bit(TWCR, *TWINT, true);
    set_bit(TWCR, *TWSTA, true);
    set_bit(TWCR, *TWEN, true);

    while ( read_volatile(TWCR) & _BV(*TWINT) ) == 0 {
        // wait until TWINT resets to zero
    }
}

pub unsafe fn stop() {
    set_bit(TWCR, *TWINT, true);
    set_bit(TWCR, *TWSTO, true);
    set_bit(TWCR, *TWEN, true);
}

pub unsafe fn write(data: u8) {
    write_volatile(TWDR, data);
    set_bit(TWCR, *TWINT, true);
    set_bit(TWCR, *TWEN, true);

    while ( read_volatile(TWCR) & _BV(*TWINT) ) == 0 {
        // wait until TWINT resets to zero
    }
}

pub unsafe fn read_ACK() -> u8 {
    set_bit(TWCR, *TWINT, true);
    set_bit(TWCR, *TWEN, true);
    set_bit(TWCR, *TWEA, true);
    while ( read_volatile(TWCR) & _BV(*TWINT) ) == 0 {
        // wait until TWINT resets to zero
    }
    read_volatile(TWDR)
}

pub unsafe fn read_NACK() -> u8 {
    set_bit(TWCR, *TWINT, true);
    set_bit(TWCR, *TWEN, true);
    while ( read_volatile(TWCR) & _BV(*TWINT) ) == 0 {
        // wait until TWINT resets to zero
    }
    read_volatile(TWDR)
}

pub unsafe fn get_status() -> u8 {
    let status: u8;
    
    // Mask status
    status = read_volatile(TWSR) & 0xF8;
    status
}
