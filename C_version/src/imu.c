#include "imu.h"
#include "usart.h"

void imu_setup(void) {
    // Check that communcation with IMU is correct
    usart_println("Initializing IMU");
    uint8_t who = i2c_read_byte(IMU_WADD,IMU_RADD,IMU_WHOAMI);
    if (who != 0b01101001) { 
        char buff[10];
        sprintf(buff,"who = %dd",who);
        panic_msg(buff); 
    }
    else {
        usart_println("IMU found");
    }

    // Initialize the acceleration sensor
    i2c_write_byte(IMU_WADD,IMU_CTRL1_XL,0b10000010); // Sample rate 1.66 kHz, 2g sensitivity, 100 Hz filter

    // Initialize gyroscope
    i2c_write_byte(IMU_WADD,IMU_CTRL2_G,0b10001000);  // Sample rate 1.66 kHz, 1000 dps sensitivity
    
    // Control register
    i2c_write_byte(IMU_WADD,IMU_CTRL3_C,0b00000100);  //  IF_INC = 1 
    
    usart_println("IMU setup complete");

}
uint8_t acc_get_status() {
    uint8_t status = i2c_read_byte(IMU_WADD,IMU_RADD,IMU_STATUS_REG);
    status = status & 0b00000001;
    return status;
}

uint8_t gyro_get_status() {
    uint8_t status = i2c_read_byte(IMU_WADD,IMU_RADD,IMU_STATUS_REG);
    status = status & 0b00000010;
    return status;
}

uint8_t temp_get_status() {
    uint8_t status = i2c_read_byte(IMU_WADD,IMU_RADD,IMU_STATUS_REG);
    status = status & 0b00000100;
    return status;
}


void imu_read(uint8_t reg, int16_t *data, int len) {
    uint8_t raw[14];
    i2c_read_multiple(IMU_WADD,IMU_RADD,reg,raw,len);

    // Convert each high/low 8-bit values to 16-bit values
    int i = 0;
    for (i = 0; i < 7; i++) {
        data[i] = (raw[(i*2)+1] << 8) | raw[i*2];
    }
}

void who_am_I(void) {
    uint8_t who = i2c_read_byte(IMU_WADD,IMU_RADD,IMU_WHOAMI);
    char buff[10];
    sprintf(buff,"who = %d",who);
    usart_println(buff);
}