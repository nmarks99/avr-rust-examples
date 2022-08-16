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
    
    i2c_master_setup(); // Initialize I2C
    imu_setup();        // Initialize the IMU 
    int16_t data[7];
    char buff[200];
    usart_println("Entering loop...");
    while(1){
        // imu_read(IMU_OUT_TEMP_L,data,14); // read temp, gyro, acc
        // _delay_ms(20);
        // sprintf(buff,"%hi\t%hi\t%hi \n",data[0],data[1],data[2]);
        // usart_println(buff);

    }    
}
