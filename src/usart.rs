use utils::*;
use atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
// use core::ptr;


pub const F_CPU: u32 = 16000000;
pub const BAUD: u32 = 9600;
pub const UBRR_VALUE:u32 =  ((F_CPU) + 8 * (BAUD)) / (16 * (BAUD)) -1;
pub const UBRRL_VALUE:u8 = (UBRR_VALUE & 0xff) as u8;
pub const UBRRH_VALUE:u8 = (UBRR_VALUE >> 8) as u8;


pub unsafe fn usart_init() {   
    // Set to 8 bit data size
    set_bit(UCSR0C,0x1,true); // UCSZ00 = 1
    set_bit(UCSR0C,0x2,true); // UCSZ01 = 2

    // Enable receiver and transitter
    set_bit(UCSR0B,0x3,true); // Tx enable
    set_bit(UCSR0B,0x4,true); // Rx enable

    // Set baudrate
    write_volatile(UBRR0H,UBRRH_VALUE);
    write_volatile(UBRR0L,UBRRL_VALUE);
}

unsafe fn tx_is_ready() -> bool {
    // Check if the UDRE0 field in UCSR0A register is 1 (ready) or 0 (not ready)
    // UDRE0 = 0x20 = 0b00100000
    if (read_volatile(UCSR0A) & UDRE0 as u8) == 0 {
         false 
    } else{ true }
}

unsafe fn rx_is_ready() -> bool {
    // Check if the RXC0 field in UCSR0A register is 1 (ready) or 0 (not ready)
    if (read_volatile(UCSR0A) & RXC0 as u8) == 0 {
        false
    } else { true }
}

unsafe fn usart_send_byte(c:char) {
    // Writes a single byte to usart buffer if its ready
    loop{ if tx_is_ready(){ break } } // do nothing until tx buffer is ready
    write_volatile(UDR0,c as u8); // when its ready, store character in tx buffer
}

pub unsafe fn read() -> char {
    loop{ if rx_is_ready(){ break } }
    read_volatile(UDR0) as char // read and return data from buffer
}

pub unsafe fn print(msg: &str) {
    for c in msg.chars() {
        usart_send_byte(c);
    }
}