#include <util/setbaud.h>
#include <avr/io.h>


#define F_CPU 16000000UL
#define BAUD 9600


void uart_init(void) {
    UBRR0H = UBRRH_VALUE;
    UBRR0L = UBRRL_VALUE;

  #if USE_2X
      UCSR0A |= _BV(U2X0);
  #else
      UCSR0A &= ~(_BV(U2X0));
  #endif

    UCSR0C = _BV(UCSZ01) | _BV(UCSZ00); /* 8-bit data */
    UCSR0B = _BV(RXEN0) | _BV(TXEN0);   /* Enable RX and TX */
}

void uart_putchar(char c) {
   loop_until_bit_is_set(UCSR0A, UDRE0); /* Wait until data register empty. */
   UDR0 = c;
}

char uart_getchar(void) {
    loop_until_bit_is_set(UCSR0A, RXC0); /* Wait until data exists. */
    return UDR0;
}



int main(void) {

    uart_init();
    
    while(1) {
    }

    return 0;
}
