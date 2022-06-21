use core::intrinsics::volatile_store;

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
    volatile_store(addr,val);

}
