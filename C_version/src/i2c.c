
#include "i2c.h"

void i2c_master_setup(void) {
    TWCR |= (1 << TWEN); // Turn on I2C module  
}



void i2c_master_start(void) {
    TWCR |= (1 << TWSTA);
    TWCR |= (1 << TWEN); // Turn on I2C module  
    // while (!(TWCR & (1 << TW)))
}
void i2c_master_restart(void); // send a RESTART signal
void i2c_master_send(unsigned char byte); // send a byte (either an address or data)
unsigned char i2c_master_recv(void); // receive a byte of data
void i2c_master_ack(int val); // send an ACK (0) or NACK (1)
void i2c_master_stop(void); // send a stop



