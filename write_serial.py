#!/usr/bin/env python3
import serial
ser = serial.Serial('/dev/ttyUSB0',baudrate=115200)


try:
    while True:
        msg = input(">> ");
        "{}".format(msg)
        ser.write(msg.encode(encoding='UTF-8'))
        ser.write(b'\n')
except KeyboardInterrupt: 
    ser.close()
    



