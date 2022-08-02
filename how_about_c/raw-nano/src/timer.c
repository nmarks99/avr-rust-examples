#include "timer.h"

// turn on timer 1, set prescaler to 64
void timer1_init(void) {
    TCCR1B = 0b11000000;
}

unsigned short get_time1(void) {
    unsigned short ticks = TCNT1;
    return ticks;
}