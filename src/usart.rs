use utils::*;
use atmega328p::*;
use core::intrinsics::{volatile_store,volatile_load};


// const MY_UBRR: u32 = (SYS_CLK/(16*BAUD)) - 1;


pub unsafe fn usart_init() {


    // set_bit(UCSR0A,0x2,false); // sets bit 1, the second bit (U2X0=0x2), to zero

    // Set to 8 bit data size
    volatile_store(UCSR0C, (1<<2) | (1<<1) );
    // set_bit(UCSR0C,0x1,true); // UCSZ00 = 1
    // set_bit(UCSR0C,0x2,true); // UCSZ01 = 2
    // set_bit(UCSR0B,0x3,false); //UCSZ02 = 0

    // Enable receiver and transitter
    volatile_store(UCSR0B, 1<<3 );
    // set_bit(UCSR0B,0x4,true); // Tx enable
    // set_bit(UCSR0B,0x5,true); // Rx enable

    // Set baudrate
    let UBRRH_VALUE:u8 = 0b00000000;
    let UBRRL_VALUE:u8 = 0b01100111;
    volatile_store(UBRR0H,UBRRH_VALUE);
    volatile_store(UBRR0L,UBRRL_VALUE);

}






// pub unsafe fn uart_send_byte(c:u8) {
//     (!( UCSR0A & (1<<UDRE0)))
// }


pub unsafe fn write_usart(c: u8) {
    loop {    
        set_bit(PORTD, 2, true); // turn led on
        // set_bit(PORTD, 2, true); // set high
        match Some(volatile_load(UCSR0A) & 1 << volatile_load(UDRE0)) {
            Some(_c) => continue,
            None => break
        }
    }
    set_bit(PORTD, 2, false);
    volatile_store(UDR0, c);

}
