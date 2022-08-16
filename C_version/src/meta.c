#include "meta.h"
void panic(void) {
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
    PORTB |= (1 << PORTB5); // set high
    while(1){ ; }
}