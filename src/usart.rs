use utils::*;
use atmega328p::*;
use core::intrinsics::{volatile_store,volatile_load};


const BAUD: u32 = 9600;
const MY_UBRR: u32 = (SYS_CLK/(16*BAUD)) - 1;

pub unsafe fn usart_init() {
    set_bit(UCSR0A,0x2,false); // sets bit 1, the second bit (U2X0=0x2), to zero

    // Set to 8 bit data size
    set_bit(UCSR0C,0x2,true); // UCSZ00 = 1
    set_bit(UCSR0C,0x3,true); // UCSZ01 = 1
    set_bit(UCSR0B,0x3,false); //UCSZ02 = 0

    // Enable receiver and transitter
    set_bit(UCSR0B,0x4,true); // Tx enable
    set_bit(UCSR0B,0x5,true); // Rx enable
    // volatile_store(UCSR0B, (1<<*TXEN0) | (1<<*RXEN0) );
    // volatile_store(UCSR0C, (1<<0b00000001) | (1<<0b00000001));

    // Set baudrate
    volatile_store(UBRR0H,0);
    volatile_store(UBRR0L,0b00001100);

    set_bit(PORTD, 2, true);
    arduino_hal::delay_ms(800);
    set_bit(PORTD, 2, false);
    arduino_hal::delay_ms(800);
    set_bit(PORTD, 2, true);
    arduino_hal::delay_ms(800);
    set_bit(PORTD, 2, false);
    arduino_hal::delay_ms(800);
    set_bit(PORTD, 2, true);
    arduino_hal::delay_ms(800);
    set_bit(PORTD, 2, false);
    arduino_hal::delay_ms(2000);

}

pub unsafe fn write_usart(data: &char) {
    loop {    
        set_bit(PORTD, 2, true);
        // set_bit(PORTD, 2, true); // set high
        match Some(*volatile_load(&UCSR0A) & 1 << *volatile_load(&UDRE0)) {
            Some(_c) => continue,
            None => break
        }
    }
    set_bit(PORTD, 2, false);
    // set_bit(PORTD, 2, false);
    volatile_store(UDR0, *data as u8);

}