#include <avr/io.h>
#include <stdio.h>

#ifndef F_CPU
#define F_CPU 16000000UL
#endif

#include <util/delay.h>

#ifndef BAUD
#define BAUD 9600
#endif

#include <util/setbaud.h>

void uart_init(void) {
    UBRR0H = UBRRH_VALUE; // 0
    UBRR0L = UBRRL_VALUE; // 103
    UCSR0C = (1<<UCSZ01) | (1<<UCSZ00); /* 8-bit data */ 
    UCSR0B = (1 << TXEN0) | (1 << RXEN0); // Enable TX and RX
}

void usart_send_byte(char val) {
    while ( !( UCSR0A & (1<<UDRE0)) );
    UDR0 = val;
}

void usart_send_str(char *str, uint16_t len) {
    
    int i;
    for (i = 0; i < len; i++){
        usart_send_byte(str[i]);
    }

}


char read(){
    while ( !(UCSR0A & (1<<RXC0))) {}
    return UDR0;
}

int main(void) {   
    uart_init();
    DDRD |= (1 << 2);   
    usart_send_str("Begin\n",6);
    while (1) {

        char val = read();
        if (val == 'A'){
            PORTD |= (1 << PORTD2);
            _delay_ms(200);
            PORTD &= !(1 << PORTD2);        
            _delay_ms(200);
            usart_send_str("Got an A!\r\n",11);
        }
        else if (val == 'B'){
            PORTD |= (1 << PORTD2);
            _delay_ms(200);
            PORTD &= !(1 << PORTD2);        
            _delay_ms(200);
            usart_send_str("Got a B!\r\n",11);
        } 
        else{
            usart_send_str("hmm\r\n",5);
        }
    }

    return 0;
}
