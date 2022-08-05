#include "timer.h"
#include<stdio.h>


void init(void) {
    // turn on timer 1, set prescaler to 64
    TCCR1B = 3; 
}

unsigned short get_count(void) {
    unsigned short ticks = TCNT1;
    return ticks;
}

void reset(void) {
    TCNT1 = 0;
}

int overflow_flag(void) {
    if ( (TIFR1 & (1 << TOV1)) == 0 ) {
        return 0;
    }
    else {
        TIFR1 = (1 << TOV1);
        return 1;
    }
}


void delay(float ms) {
    #warning delay() is both blocking and inefficient

    // this math should not be done on the mcu...
    const float desired_ticks = (uint32_t)(ms*TICKS_PER_MS);
    const uint8_t desired_overflows = (uint8_t)(floor(desired_ticks/MAX_TICKS));
    const uint32_t remaining_ticks = (uint32_t)desired_ticks % MAX_TICKS;
    uint8_t current_overflow = 0;
    unsigned short current_ticks; 
    
    init();
    while(1) {
        current_ticks = get_count();
        
        if (desired_overflows > 0) {
            if (overflow_flag() == 1) {
                if (current_overflow < desired_overflows) {
                    current_overflow += 1;
                    reset();
                }
                else {
                    if (current_ticks >= remaining_ticks) {
                        break;
                    }
                }
            }
        } 
        else {
            if (current_ticks >= desired_ticks) {
                break;
            }
        }
    }
}


