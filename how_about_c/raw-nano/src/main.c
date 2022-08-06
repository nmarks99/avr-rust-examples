#include "timer.h"
#include <avr/interrupt.h>
#include <stdio.h>
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
    unsigned long t0;
    unsigned long t_now;
    unsigned long elap;
    char m[50];
    while (1) {  

        t0 = millis();
        _delay_ms(1000);
        t_now = millis();
        elap = t0 - t_now;
        sprintf(m,"%lu",elap); 
        overflows = 0;
        usart_println(m);
    }

    return 0;
}
