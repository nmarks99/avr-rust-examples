use core::ptr::write_volatile;

pub fn LS1(bit: u8) -> u8 {
    // Left shifts 1 the specfied amount
    1 << (bit)
}

pub unsafe fn set_bit(addr: *mut u8, bit: u8, state: bool) {
    // sets 'bit' at address 'addr' to 1 ('state' = true) or 0 ('state' = false)
    let val: u8;
    match state {
        true => val = *addr | LS1(bit),
        false => val = *addr & !LS1(bit)
    }
    write_volatile(addr,val);
}




