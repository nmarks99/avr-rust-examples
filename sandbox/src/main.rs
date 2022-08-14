#![no_std]
#![feature(abi_avr_interrupt)]
#![no_main]
#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(non_snake_case)]

// Define panic handler
use core::{panic::PanicInfo};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


use nano_hal::gpio::LED_BUILTIN;
use nano_hal::meta;
use core::ptr::write_volatile;


const RESET_VAL:u16 = (65535 - (meta::F_CPU/64)/1000) as u16;
// const TICKS_PER_MS:u8 = 250;
static mut MILLIS: u32 = 0;


// #[avr_device::interrupt(atmega328p)]
// unsafe fn TIMER1_OVF() {
//     // timer1_reset();
//     MILLIS += 1;
//     led_toggle();
// }

pub unsafe fn timer1_init() {
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    p.TC1.tccr1b.write(|w| w.bits(3u8)); // pre = 64
}

pub unsafe fn timer1_reset() {
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    p.TC1.tcnt1.write(|w| w.bits(RESET_VAL)); 
}

pub unsafe fn interrupt_init(){
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    let current_bits = p.TC1.timsk1.read().bits() as u8;
    let new_bits: u8 = current_bits | 0x01;
    p.TC1.timsk1.write(|w| w.bits(new_bits)); 
    avr_device::interrupt::enable();
}

pub unsafe fn led_toggle() { 
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    let current_bits = p.PORTB.portb.read().bits();
    let new_bits = current_bits ^ (1 << 5);
    p.PORTB.portb.write(|w| w.bits(new_bits));
}

pub unsafe fn led_set_high() {
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    let current_bits = p.PORTB.portb.read().bits();
    let new_bits = current_bits | (1 << 5);
    p.PORTB.portb.write(|w| w.bits(new_bits));
}

pub unsafe fn let_set_low() {
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    // let current_bits = p.PORTB.portb.read().bits();
    // let new_bits = current_bits & !(1 << 5);
    p.PORTB.portb.write(|w| w.bits(0x00));
}

pub unsafe fn led_set_output() {
    

}


#[no_mangle]
fn main() -> ! {
    unsafe {
        led_set_output();
        // interrupt_init();
        // timer1_init();
        led_set_high();
        // let mut c:u32 = 0; 
        loop{
            // let t0: u32 = MILLIS;
            
            // loop {
            //     let tf:u32 = MILLIS;
            //     if (tf - t0) >= 2000 {
            //         break;
            //     }
            // }
            // if c % 2 == 0 {
            //     LED_BUILTIN.high();
            // }
            // else {
            //     LED_BUILTIN.low();
            // }
            // c += 1;
        }
    } 
}






