use crate::atmega328p::*;
use core::ptr::{write_volatile,read_volatile};

const REFS0: u8 = 6;
const ADPS0: u8 = 0;
const ADPS1: u8 = 1;
const ADPS2: u8 = 2;
const ADEN: u8 = 7;

pub const ADC0: u8 = 0b00000000;
pub const ADC1: u8 = 0b00000001;
pub const ADC2: u8 = 0b00000010;
pub const ADC_VBG: u8 = 0b0001110;
pub const ADC_GND: u8 = 0b00001111;


pub unsafe fn init() {

    // Use AVcc as reference voltage
    let res:u8 = read_volatile(ADMUX) | (1 << REFS0);
    write_volatile(ADMUX, res);
   
    // Set prescaler to 128
    // 16MHz/128 = 125kHz
    let res:u8 = read_volatile(ADCSRA) | (1 << ADPS0) | (1 << ADPS1) | (1 << ADPS2) | (1 << ADEN);
    write_volatile(ADCSRA, res);
}

pub unsafe fn adc_read(channel: u8) -> u16 {
    // Select ADC channel, masking for safety
    let res: u8 = (read_volatile(ADMUX) & 0xF0) | (chan & 0x0F);
    write_volatile(ADMUX,res);

    // Set single conversion mode
    // settings ADSC to start the conversion
    let res: u8 = read_volatile(ADCSRA) | (1 << ADSC);
    write_volatile(ADCSRA, res);

    let stat: u8 = read_volatile(ADCSRA) & (1 << ADSC); 
    while stat == 1u8 {} // wait for conversion to finish (gets set back to 0)

    // Return value from the ADC
    read_volatile(ADC)

}