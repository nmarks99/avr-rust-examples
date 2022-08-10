#include "timer.h"
#include <avr/interrupt.h>
#include <stdio.h>
// PORTB |= (1 << PORTB5); // set high
// PORTB &= ~(1 << PORTB5); // set low
// DDRB |= (1 << DDB5); // set pin B5 as ouptut


/* ISR(TIMER1_OVF_vect) { */
    /* timer1_reset(); */
    /* MILLIS++; */
/* } */
/*  */
/* void intialize(void) { */
    /* TIMSK1 |= 0x01; */
    /* sei(); */
/* } */

void set_output(void) {
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
}
void set_low(void) {
    PORTB &= ~(1 << PORTB5); // set low
}
void set_high(void) {
    PORTB |= (1 << PORTB5); // set high
}

int main(void) {   
   
    /* intialize(); */
    /* timer1_init(); */
    /* usart_init(); */
    /* char m[30]; */
    set_output();
    set_high();
    while (1) {  
        /* unsigned long t0 = millis(); */
        /* _delay_ms(320); */
        /* unsigned long tf = millis(); */
        /* unsigned long elap = tf - t0; */
        /* sprintf(m,"%lu",elap); */
        /* usart_println(m);     */
    }

    return 0;
}
