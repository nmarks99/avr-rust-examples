#!/usr/bin/env bash
#-----------------------------------
# Flash script for arduino
#-----------------------------------


# need to have avr-gcc and avr-libc installed
# sudo apt install avr-gcc avr-libc

# also need this version of nightly for some reason
# rustup override set nightly-2021-01-07

# Check if this is a rust project
# Presumably if there is a Cargo.toml file this is rust...
if [ -f "./Cargo.toml" ]; then

    # If no arguments passed
    # Flash the elf file in ./target/avr-atmega328p/
    if [[ ! -n $1 ]]; then

        debug_flag=false
        release_flag=false
        # note that this could fail if the directory exists but the
        # elf file is not in there. This is to avoid having find the filename
        # you'll get an error later if the elf file is missing
        if [[ -d ./target/avr-atmega328p/debug/ ]]; then
            debug_flag=true
        fi

        if [[ -d ./target/avr-atmega328p/release/ ]]; then
            release_flag=true 
        fi

    
        # if both release and debug .elf files exist, flash the newer one
        if [[ $debug_flag = true ]] && [[ $release_flag = true ]]; then
            release_elf=$(ls ./target/avr-atmega328p/release/*.elf| head -1 )
            debug_elf=$(ls ./target/avr-atmega328p/debug/*.elf| head -1 )
            
            if [ "$release_elf" -nt "$debug_elf" ]; then
                elf="$release_elf"
            elif [ "$debug_elf" -nt "$release_elf" ]; then
                elf="$debug_elf"
            fi
        
        elif [[ $debug_flag = true ]]; then
            debug_elf=$(ls ./target/avr-atmega328p/debug/*.elf| head -1 )
            elf="$debug_elf"

        elif [[ $release_flag = true ]]; then
            release_elf=$(ls ./target/avr-atmega328p/release/*.elf| head -1 )
            elf="$release_elf"
        else 
            echo "No .elf file found. Please compile with 'cargo build' before flashing"
            exit 1
        fi
         
        # Flash to board with avrdude
        echo "Flashing $elf"
        avrdude -p m328p -c arduino -P /dev/ttyUSB0 -b 115200 -U flash:w:$elf

    fi

    # else
    #
        # while [[ -n $1 ]] # while not empty
        # do
            # case "$1" in
                # --release) echo "release option" ;;
                # *) echo "$1 is not a valid option" ;;
            # esac
            # shift
        # done
#
    # fi
fi









