#include "timer.h"
#include <avr/interrupt.h>
#include <stdio.h>

ISR(TIMER1_OVF_vect) {
    timer1_reset();
    MILLIS++;
}

void intialize(void) {
    TIMSK1 |= 0x01;
    sei();
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


int main(void) {   
    led_set_output();
    led_set_high();
   
    intialize(); 
    timer1_init();
    
    unsigned long delay_time_ms = 2000;
    while (1) {  
        
        unsigned long t0 = millis();
        
        while (1) {
            unsigned long tf = millis();
            unsigned long elap = tf - t0;
            
            if (elap >= delay_time_ms) {
                break;
            }
        }
        PORTB ^= (1 << PORTB5);
    }

    return 0;
}
