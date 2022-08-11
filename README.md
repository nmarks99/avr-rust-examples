# nano-hal
A simple hardware abstraction layer written from in Rust
for the ATMega328p microcontroller/Arduino Nano board. 

## What this project tries to be:
- As easy as possible to understand, especially for someone coming from 
embedded C. Rust in a "C style". True rustaceans are shaking at this...
- Binary sizes comparable to what you'd get in C
- Lower level than Arduino framework, but still with plenty
of useful abstractions.

## What this project does not try to be 
- Perfectly idiomatic and safe rust code. Often safety and "rustyness" will be 
sacrificed for readability, simplicity, compiled binary size, and likeness to C.


# TODO:
- [ ] Figure out a way to write unit tests in embedded no_std environment
- [x] Write digital pins
- [x] Write USART
- [x] Read USART
- [x] Read digital pins
- [ ] Use timers
- [ ] Improve timer code for flexibility
- [ ] Improve GPIO code for easy of use and performance
- [ ] Read analog pin
- [ ] Read I2C device
- [ ] Control PWM pin

