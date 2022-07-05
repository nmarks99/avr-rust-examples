use utils::*;
use atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

// pub const SCL_PRE: u8 = 1;
// pub const SCL_FREQ: u32 = F_CPU / 16 + 2*(TWBR) * (SCL_PRE)

pub unsafe fn i2c_init() {
    // Set SCL to 400kHz
    write_volatile(TWSR, 0x00);
    write_volatile(TWBR, 0x0C); // 0x0C = 12, 16000000/(16+(2*12)) = 400 kHz
    
    // Enable TWI
    write_volatile(TWCR, _BV(TWEN));
}