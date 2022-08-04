





fn main() {

    let MAX_TICKS:u32 = 65536;
    let ms: f32 = 250.0; 
    let desired_ticks: u32 = (ms*250.0) as u32;
    let desired_overflows: u8 = (((desired_ticks/MAX_TICKS) as f32).floor() ) as u8;
    let remaining_ticks: u16 =  (desired_ticks % MAX_TICKS) as u16;
    println!("{}\n",desired_ticks);
    println!("{}\n",desired_overflows);
    println!("{}\n",remaining_ticks);
    

}
