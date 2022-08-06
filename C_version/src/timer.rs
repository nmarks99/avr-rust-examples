use crate::atmega328p::*;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
use micromath::F32Ext;


// For now this is just timer1 which is a 16 bit timer
pub struct Timer {
    pre: u16
}

impl Timer {
   
    pub unsafe fn init(&self) {

        let pre:u8 = match &self.pre{ 
            1 => 1u8,
            8 => 2u8,
            64 => 3u8,
            256 => 4u8,
            1024 => 5u8,
            _ => loop {}
        };

        write_volatile(TCCR1B,pre); // set prescaler to 64 
    }

    pub unsafe fn get_count(&self) -> u16 {
        read_volatile(TCNT1)
    }

    pub unsafe fn reset(&self) {
        write_volatile(TCNT1,0u16);
    }

    pub unsafe fn overflow_flag(&self) -> bool {
        if (read_volatile(TIFR1) & (1 << *TOV1)) == 0 {
            false    
        }
        else {
            write_volatile(TIFR1, 1 << *TOV1);
            true 
        }
        
    }

}

pub const T1: Timer = Timer{pre: 64};
pub const MAX_TICKS: u32 = 65536;
pub const TICKS_PER_MS: u8 = 250;

// #[macro_export]
// macro_rules! delay {
//     ($ms:expr) => {
//         let desired_ticks: u32 = (ms*TICKS_PER_MS) as u32;
//         let desired_overflows: u8 = (((desired_ticks/MAX_TICKS) as f32).floor() ) as u8;
//         let remaining_ticks: u32 =  desired_ticks % MAX_TICKS;
//         let mut current_overflow: u8 = 0; 
//         let mut current_ticks: u32;
        
//         T1.init();
        
//         loop {

//             current_ticks = T1.get_count() as u32;
            
//             if desired_overflows > 0  {
//                 if T1.overflow_flag() == true {
//                     if current_overflow < desired_overflows{
//                         current_overflow += 1;
//                         T1.reset();
//                     }
//                     else {
//                     if current_ticks >= remaining_ticks {
//                             break
//                     }
//                     }
//                 } 
//             }

//             else {
//                 if current_ticks >= desired_ticks {
//                     break
//                 }
//             }
            
//         }

        
//     };
//     () => {
//         loop {};
//     };
// }


pub unsafe fn delay(ms: f32) {
    
    // let desired_ticks = 62500;
    // let desired_overflows = 0;
    // let remaining_ticks = 0;

    let desired_ticks: u32 = (ms*TICKS_PER_MS) as u32;
    let desired_overflows: u8 = (((desired_ticks/MAX_TICKS) as f32).floor() ) as u8;
    let remaining_ticks: u32 =  desired_ticks % MAX_TICKS;
    let mut current_overflow: u8 = 0; 
    let mut current_ticks: u32;

    T1.init();
    
    loop {

        current_ticks = T1.get_count() as u32;
        
        if desired_overflows > 0  {
            if T1.overflow_flag() == true {
                if current_overflow < desired_overflows{
                    current_overflow += 1;
                    T1.reset();
                }
                else {
                if current_ticks >= remaining_ticks {
                        break
                }
                }
            } 
        }

        else {
            if current_ticks >= desired_ticks {
                break
            }
        }
        
    }
}
