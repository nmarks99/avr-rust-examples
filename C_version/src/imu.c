#include "imu.h"
#include "usart.h"

struct ImuStatus;

void imu_setup(void) {
    // unsigned char who;
    // Check that communcation with IMU is correct
    
    // who = read_byte(IMU_WADD,IMU_RADD,IMU_WHOAMI);
    // if (who != 0b1101001) { 
    //     char buff[10];
    //     sprintf(buff,"who = %x",who);
    //     panic_msg(buff); 
    // }
    ImuStatus s = {0,0,0};
    imu_get_status(&s);
    if (s.acc != 1){
        panic_msg("Accelerometer status 0");
    } 

    // Initialize the acceleration sensor
    i2c_write_byte(IMU_WADD,IMU_CTRL1_XL,0b10000010); // Sample rate 1.66 kHz, 2g sensitivity, 100 Hz filter

    // Initialize gyroscope
    i2c_write_byte(IMU_WADD,IMU_CTRL2_G,0b10001000);  // Sample rate 1.66 kHz, 1000 dps sensitivity

    // Control register
    i2c_write_byte(IMU_WADD,IMU_CTRL3_C,0b00000100);  //  IF_INC = 1 

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



void imu_get_status(ImuStatus* status) {
    uint8_t mask = 0b00000111;
    uint8_t _stat = i2c_read_byte(IMU_WADD,IMU_RADD,IMU_STATUS_REG);
    _stat = _stat & mask;
    // struct ImuStatus status;
    if ((_stat & (1 << 0)) == (1 << 0)) {
        status->acc = 1;
    }
    else {
        status->acc = 0;
    }
    
    if ((_stat & (1 << 1)) == (1 << 1)) {
        status->gyro = 1;
    }
    else {
        status->gyro = 0;
    }
   
    if ((_stat & (1 << 2)) == (1 << 2)) {
        status->temp = 1;
    }
    else {
        status->temp = 0;
    }
    return status;
}
