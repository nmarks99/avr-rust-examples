#ifndef __USART__H__
#define __USART__H__

#include<string.h>

#ifndef F_CPU
#define F_CPU 16000000UL
#endif

#include <util/delay.h>

#ifndef BAUD
#define BAUD 9600

#endif

#include <util/setbaud.h>
#include<avr/io.h>

// functions
void usart_init(void);
void usart_send_byte(char val);
void usart_print(char *str);
void usart_println(char *str);
void usart_newline();
char read_byte(void);
void read_str(char *buff);

#endif