#![feature(abi_avr_interrupt)]
#![no_std]
#![feature(llvm_asm)]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod atmega328p;
pub mod meta; 
pub mod usart;
pub mod i2c;
pub mod gpio;
pub mod timer;
pub mod adc;
