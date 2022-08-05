#include "usart.h"

void usart_init(void) {
    UBRR0H = UBRRH_VALUE; // 0
    UBRR0L = UBRRL_VALUE; // 103
    UCSR0C = (1<<UCSZ01) | (1<<UCSZ00); /* 8-bit data */ 
    UCSR0B = (1 << TXEN0) | (1 << RXEN0); // Enable TX and RX
}

void usart_send_byte(char val) {
    while ( !( UCSR0A & (1<<UDRE0)) );
    UDR0 = val;
}

void usart_print(char *str) {
    uint16_t i;
    uint16_t len = strlen(str); 
    for (i = 0; i < len; i++){
        usart_send_byte(str[i]);
    }
}

void usart_println(char *str) {
    uint16_t i;
    uint16_t len = strlen(str); 
    for (i = 0; i < len; i++){
        usart_send_byte(str[i]);
    }
    usart_send_byte('\n');
}


char read_byte(void){
    while ( !(UCSR0A & (1<<RXC0))) {}
    return UDR0;
}

void read_str(char *buff) {
    int i = 0;
    char EOL_char = '\n';
    while(1){
        char temp_byte = read_byte();
        if (temp_byte != EOL_char) {
            buff[i] = temp_byte;
            i ++;
        } else { break; }
    }
}
