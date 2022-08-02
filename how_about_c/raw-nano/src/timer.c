#include "timer.h"

void timer1_init(void) {
    // turn on timer 1, set prescaler to 64
    TCCR1B = 0b11000000;
}

unsigned short get_time1(void) {
    unsigned short ticks = TCNT1;
    return ticks;
}