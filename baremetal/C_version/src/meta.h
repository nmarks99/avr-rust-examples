#include <avr/io.h>
#include "usart.h"

void panic(void);
void panic_msg(char *msg);
void led_set_output(void);
void led_set_low(void);
void led_set_high(void);
void led_toggle(void);