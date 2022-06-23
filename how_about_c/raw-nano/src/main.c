#include <avr/io.h>
#include <stdio.h>
#include <string.h>


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


char read_byte(){
    while ( !(UCSR0A & (1<<RXC0))) {}
    return UDR0;
}

void read_str(char *buff,char EOL_char) {
    size_t i = 0;
    while(1){
        char temp_byte = read_byte();
        if (temp_byte != EOL_char) {
            buff[i] = temp_byte;
            i ++;
        } else { break; }
    }
}

int main(void) {   
    uart_init();
    DDRD |= (1 << 2);   
    usart_send_str("Begin\n",6);
    char buff[100];

    while (1) {
        read_str(buff,'\0');
        usart_send_str(buff,strlen(buff));
        _delay_ms(500);
    }

    return 0;
}
