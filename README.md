# nano-hal
A simple hardware abstraction layer written from in Rust
for the ATMega328p microcontroller/Arduino Nano board. 

My original plan was to do this entirely "from scratch" meaning 
I wouldn't use any libraries or anything however I am now considering 
using avr_device crate...

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



