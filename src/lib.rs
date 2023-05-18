use std::time::{SystemTime};
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

#[no_mangle]
pub extern "C" fn print_hello() {
    println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn multiply_by_2(n: u8) -> u8{
    n * 2
}

#[no_mangle]
pub extern "C" fn print_42(){
    println!("42")
}

#[no_mangle]
pub extern "C" fn time(){
    let now = SystemTime::now();
    println!("{:?}", now)
}

#[no_mangle]
pub extern "C" fn write_to_file() -> io::Result<()> {
    let f = File::create("output")?;
    {
        let mut writer = BufWriter::new(f);

        // write a byte to the buffer
        writer.write(&[multiply_by_2(2)])?;

    } // the buffer is flushed once writer goes out of scope

    Ok(())
}

#[no_mangle]
pub extern "C" fn multiply(left: f32, right: f32) -> f32 {
    left * right
}

#[no_mangle]
pub extern "C" fn divide_by_0(n: u8) -> u8{
    n / 1
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn to_file() {
//        let _result =  write_to_file();
        
//     }
// }
