#include "timer.h"
#include <avr/interrupt.h>
#include <stdio.h>
#include "usart.h"
#include "meta.h"
#include "i2c.h"
#include "imu.h"


int main(void) {   
    usart_init();
    usart_println("Starting...");
    DDRB |= (1 << DDB5); // set pin B5 as ouptut
    
    i2c_master_setup(); // Initialize I2C
    imu_setup();        // Initialize the IMU 
    char buff[200];
    // usart_println("Entering loop...");
    // int16_t data[7];
    while(1){
        uint8_t s = acc_get_status();
        sprintf(buff,"%d",s);
        usart_println(buff);
        // if (temp_get_status()){
        //         usart_print("Temp: OK\t");
        // }
        // else {
        //     usart_print("Temp: BAD\t");
        // }

        // if (gyro_get_status()){
        //     usart_print("gyro: OK\t");
        // }

        // else {
        //     usart_print("gyro: BAD\t");
        // }

        // if (acc_get_status()){
        //     usart_print("acc: OK");
        // }
        // else {
        //     usart_print("acc: BAD");
        // }
        // usart_print("\n"); 
        _delay_ms(500);        
    }
}    
