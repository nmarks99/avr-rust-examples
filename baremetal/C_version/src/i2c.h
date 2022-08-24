#ifndef I2C_H__
#define I2C_H__

#include <avr/io.h>
#include "meta.h"

// I2C status codes
#define I2C_START 0x08
#define I2C_RESTART 0x10
#define I2C_WRITE_ADDR_ACK 0x18
#define I2C_WRITE_ADDR_NACK 0x20
#define I2C_WRITE_DATA_ACK 0x28
#define I2C_WRITE_DATA_NACK 0x30

#define I2C_READ_ADDR_ACK 0x40
#define I2C_READ_ADDR_NACK 0x48
#define I2C_READ_DATA_ACK 0x50
#define I2C_READ_DATA_NACK 0x58

#define I2C_ERROR 0x38
#define I2C_NONE 0xF8


// Functions
void i2c_master_setup(void); // set up I2C as master
void i2c_master_start(void); // send a start signal
void i2c_master_send(uint8_t byte); // send a byte
uint8_t i2c_master_read_ack(void); // Read data, send ack
uint8_t i2c_master_read_nack(void); // Read data, don't send ack
void i2c_master_stop(void); // send a stop
uint8_t i2c_master_get_status(void); 
void i2c_write_byte(uint8_t Wadd, uint8_t reg, uint8_t value);
uint8_t i2c_read_byte(uint8_t Wadd, uint8_t Radd, uint8_t reg);
void i2c_read_multiple(uint8_t Wadd, uint8_t Radd, uint8_t reg, uint8_t *raw, int len);

#endif
