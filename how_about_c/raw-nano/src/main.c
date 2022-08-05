#include "timer.h"


int main(void) {   
    init();
    DDRB |= (1 << DDB5); // set pin B5 as ouptut

    while (1) { 

        PORTB |= (1 << PORTB5);
        delay(1000);
        PORTB &= ~(1 << PORTB5);
        delay(1000);
    } 
    return 0;
}
