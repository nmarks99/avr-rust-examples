#include "usart.h"
#include "timer.h"


int main(void) {   
    timer1_init();
    DDRB |= (1 << DDB5); // set pin B5 as ouptut

    while (1) {

        if (get_time() >= 12500) {
            PORTB ^= (1 << PORTB5);
            TCNT1 = 0; // register 0x84
        }

    } 
    return 0;
}
