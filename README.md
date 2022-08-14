# nano-hal
A simple hardware abstraction layer written in Rust
for the ATMega328p microcontroller/Arduino Nano board. 

## What this project tries to be:
- As easy as possible to understand, especially for someone coming from 
embedded C. Rust in a "C style".(True rustaceans are shaking at this...)
- Binary sizes comparable to what you'd get in C
- Lower level than Arduino framework, but still with plenty
of useful abstractions. Abstractions should be simple and 
by looking through the source code, it should quickly become
clear what they are doing under the hood.

## What this project does not try to be 
- Perfectly idiomatic rust code. Often "rustyness" will be 
sacrificed for readability, simplicity, compiled binary size
and likeness to embedded C code.
- Very memory safe. Using Rust will make for safer code than a 
typical embedded C library however don't be suprised at all the
unsafe blocks. Safety is readily sacrificed for readability and simplicity 
in many instances.

## Who asked for this?
Probably just me. I generally like Rust a lot, but I also like C a lot. 
I do not like C++. I want a framework for programming
Arduino boards (I chose the Arduino Nano/ATMega328P since I tend to use that
board most frequently in my projects) with the bare minimum amount of abstractions
that still make progamming easy and enjoyable. Nothing too abstract like the Arduino
setup() and loop() functions. I also love cargo and its nice to have a build system
like that. Memory safety is an added bonus and I'll try and use it where possible,
but in my experience programming microcontrollers, memory safety issues have rarely
come up so I'm not terribly worried about it.

Crates like avr_hal look cool, however there is barely any documentation 
and the code, although safe, is very very cryptic to me. It takes lots of
digging to find where registers are being read and written to. I also hate to have
lines of code that are calling functions and methods that are so heavily nested it 
looks like `Thing::some_thing_else::some_struct().some_method().take().unwrap()`. Like cmon,
that's terribly ugly. I'm going to try and do better with this library but no promises,
I may just abondon this whole thing very quickly and use avr_hal...or go back to C.

# TODO:
- [x] Write digital pins
- [x] Write USART
- [x] Read USART
- [x] Read digital pins
- [x] Implement millis() function
- [ ] Read analog pin with ADC
- [ ] Refactor timer code to incorporate timers 0 and 2
- [ ] Figure out a way to write unit tests in embedded no_std environment
- [ ] Improve GPIO code for easy of use and performance
- [ ] Read I2C device
- [ ] Control PWM pin

