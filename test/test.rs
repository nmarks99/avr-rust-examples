
fn main() {

    const SYS_CLK: i32 = 1000000;
    const BAUD: i32 = 921600;
    const MY_UBRR: i32 = (SYS_CLK/(16*BAUD)) - 1;

    println!("MY_UBRR = {}",MY_UBRR);
    println!("BAUD = {}",BAUD);

}