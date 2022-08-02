#include "usart.h"


uint8_t read_pin_d2() {
    if ( (PINB & (1 << PINB4)) == (1 << PINB4) ) {
        return 1;
    } else {
        return 0;
    }
}



int main(void) {   
    usart_init();
    // DDRD |= (1 << 2);   
    DDRB &= ~(1 << PINB4);
    usart_send_str("Begin\n",6);

    // char buff[100];
    while (1) {
    
        myDelay(1.0);
        usart_send_str("hi\n",3);
    
    }

    return 0;
}
