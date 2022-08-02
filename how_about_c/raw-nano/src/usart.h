#ifndef __USART__H__

#include<avr/io.h>
#ifndef F_CPU
#define F_CPU 16000000UL
#endif

#include <util/delay.h>

#ifndef BAUD
#define BAUD 9600
#endif

#include <util/setbaud.h>

// functions
void usart_init(void);
void usart_send_byte(char val);
void usart_send_str(char *str, uint16_t len);
char read_byte(void);
void read_str(char *buff);

#endif