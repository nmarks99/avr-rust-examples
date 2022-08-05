#include "timer.h"
// PORTB |= (1 << PORTB5); // set high
// PORTB &= ~(1 << PORTB5); // set low

int main(void) {   
    init();
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
    usart_init();

    while (1) { 
        delay(2000);
        usart_println("hi"); 
    }

    return 0;
}
