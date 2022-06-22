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

int uart_putchar(char, FILE*);
void uart_init();

FILE output = FDEV_SETUP_STREAM(uart_putchar, NULL, _FDEV_SETUP_WRITE);

void uart_init(void) {
    UBRR0H = UBRRH_VALUE; // 0
    UBRR0L = UBRRL_VALUE; // 103
    UCSR0C = (1<<UCSZ01) | (1<<UCSZ00); /* 8-bit data */ 
    UCSR0B = (1<<TXEN0);   /* Enable TX */    
}

int uart_putchar(char c, FILE *stream) {
    if (c == '\n') {
        uart_putchar('\r', stream);
    }
    loop_until_bit_is_set(UCSR0A, UDRE0);
    UDR0 = c;
    return 0;
}



void usart_send_byte() {
    int val = ( UCSR0A & (1<<UDRE0));
    while ( !( UCSR0A & (1<<UDRE0)) );
    UDR0 = val;
}


int main(void) {   
    uart_init();
    stdout = &output;
                
    char c = 'h';
    
    while (1) {
        // printf("%lu\t%lu\n",UBRRH_VALUE,UBRRL_VALUE);
        usart_send_byte();
        _delay_ms(500);    
    }

    return 0;
}