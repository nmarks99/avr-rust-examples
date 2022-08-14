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


// use nano_hal::gpio::*;
// use avr_device::atmega328p;
use nano_hal::meta;
// use nano_hal::timer;
use nano_hal::gpio;

const RESET_VAL:u16 = (65535 - (meta::F_CPU/64)/1000) as u16;
const TICKS_PER_MS:u8 = 125;
static mut MILLIS_COUNTER: u32 = 0;


#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER1_OVF() {
    avr_device::interrupt::free(|_cs| {
        gpio::LED_BUILTIN.set_output();
        gpio::LED_BUILTIN.high();
        // let p = avr_device::atmega328p::Peripherals::take().unwrap();
        // p.TC1.tcnt1.write(|w| w.bits(RESET_VAL));
        MILLIS_COUNTER += 1;
    })
}

fn millis_init() {
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    let tc1 = p.TC1; 
    tc1.tccr1a.write(|w| unsafe{w.bits(0x0)}); 
    // tc1.ocr1a.write(|w| unsafe { w.bits(TICKS_PER_MS as u16) });
    tc1.tccr1b.write(|w| unsafe{w.bits(0b00000011)});
    tc1.timsk1.write(|w| unsafe{w.bits(0b00000001)});
    unsafe {MILLIS_COUNTER = 0;}

}

pub unsafe fn millis() -> u32 {
    // meta::cli(); 
    MILLIS_COUNTER*TICKS_PER_MS as u32
}



#[no_mangle]
fn main() -> ! {
    unsafe{ gpio::LED_BUILTIN.set_output(); }
    let p = avr_device::atmega328p::Peripherals::take().unwrap();
    
    // let t1 = timer::Timer { pre: 64};
    // unsafe{ t1.init(); } // initialize timer1
    // p.TC1.tccr1b.write(|w| unsafe{w.bits(3)});

    
    // enable timer overflow interrupt 
    // p.TC1.timsk1.write(|w| unsafe{w.bits(0b00000001)}); 
    unsafe{ avr_device::interrupt::enable(); }
    millis_init();


    // let p = atmega328p::Peripherals::take().unwrap();
    // p.PORTB.ddrb.write(|w| unsafe{w.bits(0xFF)}); // set as output
    // p.PORTB.portb.write(|w| unsafe{w.bits(0x0)}); 
    loop {
        loop{

            if p.TC1.tifr1.read().tov1().bit_is_set() {
                unsafe{gpio::LED_BUILTIN.low();}
            }
            else {
                unsafe{gpio::LED_BUILTIN.high();}
                break;
            }
        }

    }

    // unsafe {
    //     let mut count = 0;
    //     loop {
    //         count += 1;                
    //         let t0 = millis();
    //         loop {

    //             let tf = millis();
    //             if tf - t0 >= 2000 {
    //                 if count % 2 == 0{
    //                     led.low()
    //                 }
    //                 else {
    //                     led.high()
    //                 }
    //                 break;
    //             }

                
    //         }


    //     }
    // }
}






