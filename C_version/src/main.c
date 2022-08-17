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
    // char buff[200];
    // usart_println("Entering loop...");
    // int16_t data[7];
    while(1){
        // if (acc_get_status() && gyro_get_status() && temp_get_status()) {
        //     usart_println("All good");
        //     // imu_read(0x20,data,14);
        //     // sprintf(buff,"%d,%d,%d\t%d,%d,%d",data[1],data[2],data[3],data[4],data[5],data[6]);
        //     // usart_println(buff);
        // } 
        // else {
        //     if (temp_get_status()){
        //         usart_print("Temp: OK\t");
        //     }
        //     else {
        //         usart_print("Temp: BAD\t");
        //     }

        //     if (gyro_get_status()){
        //         usart_print("gyro: OK\t");
        //     }

        //     else {
        //         usart_print("gyro: BAD\t");
        //     }

        //     if (acc_get_status()){
        //         usart_print("acc: OK");
        //     }
        //     else {
        //         usart_print("acc: BAD");
        //     }
        //     usart_print("\n");

        // }
        // _delay_ms(250);

        
    }    
}
