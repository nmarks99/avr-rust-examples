#include "meta.h"

#define ADC0 0b00000000
#define ADC1 0b00000001
#define ADC2 0b00000010
// ... and so on up to ADC8
#define ADC_VBG 0b00001110
#define ADC_GND 0b00001111

void adc_init(void);
uint16_t adc_read(uint8_t channel);