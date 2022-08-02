# nano-hal
A simple hardware abstraction layer written from scratch in Rust
for the ATMega328p microcontroller/Arduino Nano board. 

You'll notice I say from scratch, then go right ahead and import the aruino_hal crate...
This is only because I can't figure out how to get it to compile without it.
When I try and compile with avr-gcc without the arduino_hal crate, I get a linker error.

# TODO:
- [x] Write digital pins
- [x] Write USART
- [x] Read USART
- [x] Read digital pins
- [ ] User timers
- [ ] Read analog pin
- [ ] Read I2C device
- [ ] Control PWM pin
