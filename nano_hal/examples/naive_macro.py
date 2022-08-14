import os

main_rs_path = "src/main.rs"

preamble = ''' 
#![no_std]
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
'''

def main():   
    new_main = ""
    with open(main_rs_path, "r") as f:
        for i in preamble:
            new_main = "".join([new_main,i])
        for j in f: 
            new_main = "".join([new_main,j])


    os.system(f"rm {main_rs_path} && touch {main_rs_path} && echo '{new_main}' >> {main_rs_path}")
    


if __name__ == "__main__":
    main()