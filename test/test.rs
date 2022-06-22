

fn main() {
    let F_CPU: u32 = 16000000;
    let BAUD: u32 = 9600;
    let UBRR_VALUE:u32 =  ((F_CPU) + 8 * (BAUD)) / (16 * (BAUD)) -1;
    let UBRRL_VALUE:u8 = (UBRR_VALUE & 0xff) as u8;
    let UBRRH_VALUE:u8 = (UBRR_VALUE >> 8) as u8;
    println!("{:?}\n{:?}",UBRRL_VALUE,UBRRH_VALUE);
}