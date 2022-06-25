const BUFF_SIZE: usize = 10;

fn fill_buffer(buff: &mut [u8]){

    for i in 0..BUFF_SIZE {
        buff[i] = i as u8;
    }

}


fn main() {

    let mut buff: [u8;BUFF_SIZE] = [0u8;BUFF_SIZE];

    fill_buffer(&mut buff);

    println!("{:?}",buff);

}
