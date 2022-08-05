#ifndef __TIMER_H__
#include <math.h>
#include <avr/io.h>
#include "usart.h"

#define MAX_TICKS 65536
#define TICKS_PER_MS 250




void delay(float ms);
void init(void);
unsigned short get_count(void);
void reset(void);
int overflow_flag(void);



#endif

