#ifndef IMU__H__
#define IMU__H__

#include <avr/io.h>         
#include "i2c.h"
#include "meta.h"

#define IMU_WHOAMI 0x0F     // Address for WHOAMI register
#define IMU_STATUS_REG 0x20 // Status register address
#define IMU_WADD 0b11010100 // Write address
#define IMU_RADD 0b11010101 // Read address
#define IMU_CTRL1_XL 0x10   // Acceleration control register
#define IMU_CTRL2_G 0x11    // Gyroscope control register
#define IMU_CTRL3_C 0x12    // Read from multiple registers in a row
#define IMU_OUTX_L 0x28 // Acc out X,l
#define IMU_OUTX_H 0x29 // Acc out, X,h 
#define IMU_OUT_TEMP_L 0x20 // Temperature


void imu_setup(void);
void imu_read(uint8_t reg, int16_t *data, int len);
void who_am_I(void);
uint8_t acc_get_status(void);
uint8_t gyro_get_status(void);
uint8_t temp_get_status(void);


#endif