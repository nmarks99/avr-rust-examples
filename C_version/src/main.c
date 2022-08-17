#include "timer.h"
#include <avr/interrupt.h>
#include <stdio.h>
#include "usart.h"
#include "meta.h"
#include "i2c.h"
#include "imu.h"



// void led_set_output(void) {
//     DDRB |= (1 << DDB5); // set pin B5 as ouptut
// }
// void led_set_low(void) {
//     PORTB &= ~(1 << PORTB5); // set low
// }
// void led_set_high(void) {
//     PORTB |= (1 << PORTB5); // set high
// }
// void led_toggle(void) {
//     PORTB ^= (1 << PORTB5);
// }


// ISR(TIMER1_OVF_vect) {
//     /* timer1_reset(); */
//     MILLIS++;
//     led_toggle();
// }

// void intialize(void) {
//     TIMSK1 |= 0x01;
//     sei();
// }



int main(void) {   
    usart_init();
    usart_println("Starting...");
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
    
    i2c_master_setup(); // Initialize I2C
    imu_setup();        // Initialize the IMU 
    char buff[200];
    usart_println("Entering loop...");
    int16_t data[7];
    while(1){
        
        imu_read(0x20,data,14);
        sprintf(buff,"%hi,%hi,%hi\t%hi,%hi,%hi",data[1],data[2],data[3],data[4],data[5],data[6]);
        usart_println(buff);
        _delay_ms(250);
        
    }    
}
