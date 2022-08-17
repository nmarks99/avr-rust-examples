#include "adc.h"

void adc_init(void) {
    // Connect capacitor between AREF and GND to reduce noise

    ADMUX |= (1 << REFS0); // use AVcc as reference voltage

    // Set prescaler to 128, enable ADC
    // 16MHz/128 = 125kHz 
    ADCSRA |= (1 << ADPS0) | (1 << ADPS1) | (1 << ADPS2) | (1 << ADEN);
}

uint16_t adc_read(uint8_t channel){ 

    // Select ADC channel, masking for safety
    ADMUX = (ADMUX & 0xF0) | (channel & 0x0F); 
    
    // Set single conversion mode 
    // setting ADSC to start the conversion
    ADCSRA |= (1<<ADSC); 

    // Wait for conversion to finish
    // Will switch back to 0 when its done
    while( ADCSRA & (1 << ADSC) );

    // Return value from ADC 
    return ADC;

}
