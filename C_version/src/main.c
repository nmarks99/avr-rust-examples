#include <string.h>
#include <stdio.h>
#include <stdint.h>
#include <math.h>
#include "i2c.h"
#include "imu.h" 


int main(void) {
    usart_init();
    i2c_master_setup();

    char buff[50]; 
    uint8_t who = i2c_read_byte(IMU_WADD,IMU_RADD,IMU_WHOAMI);
    sprintf(buff,"%d",who); 
    // if (who != 0b1101001) { 
    //     char buff[10];
    //     sprintf(buff,"who = %x",who);
    //     panic_msg(buff); 
    // } 

}