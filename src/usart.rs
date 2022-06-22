// use utils::*;
use atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
// use core::ptr;


pub unsafe fn usart_init() {
    // set_bit(UCSR0A,0x2,false); // sets bit 1, the second bit (U2X0=0x2), to zero

    // Set to 8 bit data size
    write_volatile(UCSR0C, (1 << 2) | (1 << 1) );
    // set_bit(UCSR0C,0x1,true); // UCSZ00 = 1
    // set_bit(UCSR0C,0x2,true); // UCSZ01 = 2
    // set_bit(UCSR0B,0x3,false); //UCSZ02 = 0

    // Enable receiver and transitter
    write_volatile(UCSR0B, 1 << 3 );
    // set_bit(UCSR0B,0x4,true); // Tx enable
    // set_bit(UCSR0B,0x5,true); // Rx enable

    // Set baudrate
    let UBRRH_VALUE:u8 = 0b00000000;
    let UBRRL_VALUE:u8 = 0b01100111;
    write_volatile(UBRR0H,UBRRH_VALUE);
    write_volatile(UBRR0L,UBRRL_VALUE);
}

unsafe fn is_ready() -> bool {
    // Check if the UDRE0 field in UCSR0A register is 1 (ready) or 0 (not ready)
    // UDRE0 = 0x20 = 0b00100000
    if (read_volatile(UCSR0A) & UDRE0 as u8) == 0 {
         false 
    } else{ true }
}

unsafe fn usart_send_byte(c:char) {
    loop{ if is_ready() {break} } // do nothing until tx buffer is ready
    write_volatile(UDR0,c as u8); // when its ready, store character in tx buffer
}

pub unsafe fn usart_println(msg: &str) {
    // set_bit(PORTD,2,true);
    for c in msg.chars() {
        usart_send_byte(c);
    }
    usart_send_byte('\r');
    usart_send_byte('\n');
}