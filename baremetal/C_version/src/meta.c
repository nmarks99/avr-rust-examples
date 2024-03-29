#include "meta.h"

void panic(void) {
    // Panics, sets builtin led high, loops forever
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
    PORTB |= (1 << PORTB5); // set high
    while(1){ ; }
}

void panic_msg(char *msg) {
    // Panics, loops forever, and prints message over serial
    // usart_init();
    usart_println(msg);
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
    PORTB |= (1 << PORTB5); // set high
    while(1){ ; }
}


void led_set_output(void) {
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
}
void led_set_low(void) {
    PORTB &= ~(1 << PORTB5); // set low
}
void led_set_high(void) {
    PORTB |= (1 << PORTB5); // set high
}
void led_toggle(void) {
    PORTB ^= (1 << PORTB5);
}

