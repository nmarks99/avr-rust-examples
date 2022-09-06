
# For now this shouldn't be used because for some reason 
# the binaries it produces are bigger than platformio

SRC_DIR := src
SRC := $(wildcard $(SRC_DIR)/*.c)	

build: 
	avr-gcc -Os -mmcu=atmega328p $(SRC) -o out
flash:
	avr-objcopy -O ihex -R .eeprom out out.hex

	avrdude -F -V -c arduino -p m328p -P /dev/ttyUSB0 -b 115200 -U flash:w:out.hex