
/* EXAMPLE: Read USART, print back out whatever was recieved 

let mut buff = [None;BUFF_SIZE];
usart::readln(&mut buff);
usart::println_recieved(&mut buff);

*/


use crate::meta::*;
use crate::atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;


pub const BAUD: u32 = 9600;
pub const UBRR_VALUE:u32 =  ((F_CPU) + 8 * (BAUD)) / (16 * (BAUD)) -1;
pub const UBRRL_VALUE:u8 = (UBRR_VALUE & 0xff) as u8;
pub const UBRRH_VALUE:u8 = (UBRR_VALUE >> 8) as u8;

pub unsafe fn init() {   
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


pub unsafe fn readln(buff: &mut [Option<u8>]) {
    let EOL_char: u8 = '\n' as u8;
    for i in 0..buff.len() {
        let new_byte = read_byte();
        if new_byte != EOL_char{
            buff[i] = Some(new_byte);
        } 
        else { break }
    }
}

pub unsafe fn println(msg: &str) {
    for c in msg.chars() {
        send_byte(c as u8);
    }
    send_byte('\n' as u8);
}


pub unsafe fn print(msg: &str) {
    for c in msg.chars() {
        send_byte(c as u8);
    }
}

pub unsafe fn println_recieved(buff: &mut [Option<u8>]) {
    // Print out buffer read over usart by line
    // "None" elements at end of array are ignored
    for i in buff.iter() {
        match i {
            Some(c) => send_byte(*c),
            None => break
        }
    }
    send_byte('\n' as u8);
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


unsafe fn send_byte(c: u8) {
    // Writes a single byte to usart buffer if its ready
    loop{ if tx_is_ready(){ break } } // do nothing until tx buffer is ready
    write_volatile(UDR0,c as u8); // when its ready, store character in tx buffer
}


unsafe fn read_byte() -> u8 {
    loop{ if rx_is_ready(){ break } }
    read_volatile(UDR0)// read and return data from buffer
}

