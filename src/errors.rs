use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn string_to_int(input: String) -> Result<i32, ParseIntError> {
    return input.parse::<i32>();
}

fn read_int_from_file(file_name: &str) -> Result<i32, Box<dyn Error>> {
    let mut s = String::new();
    let _ = File::open(file_name)?.read_to_string(&mut s)?;
    let i = string_to_int(s)?;
    return Ok(i);
}

pub fn main(file_name: &str) {
    let content = read_int_from_file(file_name).expect("Did not work");
    println!("The file contains a number: {:?}", content);
}
