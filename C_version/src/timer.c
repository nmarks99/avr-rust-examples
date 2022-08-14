#include "timer.h"
#include<stdio.h>



void timer1_init(void) {
    // turn on timer 1, set prescaler to 64
    TCCR1B = 3; 
}

unsigned short timer1_get_count(void) {
    unsigned short ticks = TCNT1;
    return ticks;
}

void timer1_reset(void) {
    // Resets the timer at an offset so that it will overflow
    // every millisecond
    TCNT1 = sizeof(uint16_t) - (F_CPU/TIMER1_PRESCALER)/1000;
}

int timer1_overflow_flag(void) {
    if ( (TIFR1 & (1 << TOV1)) == 0 ) {
        return 0;
    }
    else {
        TIFR1 = (1 << TOV1);
        return 1;
    }
}


