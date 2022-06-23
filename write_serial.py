#!/usr/bin/env python3
import serial
ser = serial.Serial('/dev/ttyUSB0')


try:
    while True:
        msg = input(">> ");
        ser.write(b'{}\n'.format(msg))
except KeyboardInterrupt: 
    ser.close()
    



