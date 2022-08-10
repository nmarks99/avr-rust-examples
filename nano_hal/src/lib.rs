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

// use quote;
// #[proc_macro_attribute]
// pub fn entry(args: proc_macro::TokenStream,input: proc_macro::TokenStream) -> proc_macro::TokenStream {

//     quote::quote! (
//         #![no_std]
//         #![no_main]
//         #![feature(core_intrinsics)]
//         #![allow(dead_code)]
//         #![allow(non_snake_case)]

//         // Define panic handler
//         use core::{panic::PanicInfo};
//         #[panic_handler]
//         fn panic(_info: &PanicInfo) -> ! {
//             loop {}
//         }
//     )
//     .into()
// }