#include "timer.h"
#include <avr/interrupt.h>
// PORTB |= (1 << PORTB5); // set high
// PORTB &= ~(1 << PORTB5); // set low
// DDRB |= (1 << DDB5); // set pin B5 as ouptut


volatile unsigned short overflows = 0;

ISR(TIMER1_OVF_vect) {
    timer1_reset();
    overflows ++;
}


void stuff(void) {
    TIMSK1 |= 0x01;
    sei();
}


unsigned long millis() {
    unsigned short current_ticks = timer1_get_count();
    unsigned long total_ticks = current_ticks + (overflows * sizeof(unsigned short));
    return total_ticks/250;
    // ISR which is triggered by overflow of timer 
    // ISR then will increment the num_overflows variable
    
    // millis() is called and returns the current ticks from the timer 
    // plus (sizeof unsigned short)*num_overflows
}


int main(void) {   
    stuff();
        
    timer1_init();
    usart_init();
    unsigned long t0 = millis();
    unsigned long t_now;
    while (1) {  

        t_now = millis();
        unsigned long elapsed = t_now - t0;
        if (elapsed >= 1000) {
            usart_println("hi");
            overflows = 0;
            elapsed = 0;
            t0 = millis();
        }

        
    }

    return 0;
}
