use core::ptr::write_volatile;


// USART stuff
// pub const F_CPU: u32 = 16000000;
// pub const BAUD: u32 = 9600;
// pub const UBRR_VALUE:u8 =  ((F_CPU) + 8 * (BAUD)) / (16 * (BAUD)) -1;
// pub const UBRRL_VALUE:u8 = UBRR_VALUE & 0xff;
// pub const UBRRH_VALUE:u8 = UBRR_VALUE >> 8;

pub fn LS1(bit: u8) -> u8 {
    // Left shifts 1 the specfied amount
    1 << (bit)
}

pub unsafe fn set_bit(addr: *mut u8, bit: u8, state: bool) {
    let val: u8;
    match state {
        true => val = *addr | LS1(bit),
        false => val = *addr & !LS1(bit)
    }
    write_volatile(addr,val);
}

