#!/usr/bin/env bash
#-----------------------------------
# Flash script for arduino
#-----------------------------------


# need to have avr-gcc and avr-libc installed
# sudo apt install avr-gcc avr-libc

# also need this version of nightly for some reason
# rustup override set nightly-2021-01-07

# finally you will need avrdude



# check that its a rust project, build in release mode, flash with avrdude
if [ -f "./Cargo.toml" ]; then

    echo "Building in release mode"
    cargo build --release
    elf=$(ls ./target/avr-atmega328p/release/*.elf| head -1 )
    
    echo "Flashing $elf"
    avrdude -p m328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:$elf

else 
    echo "No Cargo.toml file found. Is this is a rust project?"
    exit 1

fi