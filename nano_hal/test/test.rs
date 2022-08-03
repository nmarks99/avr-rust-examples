

pub const fn get_clock_select(pre:u16) -> [u8;3] {
   
    let CSB: [u8;3] = match pre {
        
        1 => [0,0,1],
        8 => [0,1,0],
        64 => [0,1,1],
        256 => [1,0,0],
        1024 => [1,0,1],
        _ => panic!("Invalid prescaler value")
    };
    CSB
}


fn main() {

}
