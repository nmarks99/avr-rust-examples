#include <string.h>
#include <stdio.h>
#include <stdint.h>
#include <math.h>
#include "usart.h"
#include "adc.h"

int main(void) {
    usart_init();
    adc_init();
    double val;
    char buff[50];
    while (1) {
        
        val = (double)(10000/1024)*adc_read(ADC0);
        sprintf(buff,"Pot = %u",val);
        usart_println(buff);
        _delay_ms(200);
    }

    return 0;
}