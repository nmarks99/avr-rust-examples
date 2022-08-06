#ifndef __TIMER_H__
#include <math.h>
#include <avr/io.h>
#include "usart.h"

#define TIMER1_MAX_TICKS 65536
#define TICKS_PER_MS 250
#define TIMER1_PRESCALER 64

volatile unsigned long MILLIS;
#define millis() (MILLIS)

void timer1_init(void);
unsigned short timer1_get_count(void);
void timer1_reset(void);
int timer1_overflow_flag(void);



#endif

