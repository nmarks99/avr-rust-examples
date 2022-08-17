#ifndef I2C_H__
#define I2C_H__

#include <avr/io.h>
#include "meta.h"

#define START_SUCCESS 0x08
#define RESTART_SUCCESS 0x10
#define SLA_W_ACK_SUCCESS 0x18
#define SLA_NACK_SUCCESS 0x20
#define SLA_R_ACK_SUCCESS 0x40
#define DATA_SENT_ACK_SUCCESS 0x28
#define DATA_REC_ACK_SUCCESS 0x50


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
