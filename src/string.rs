#![deny(clippy::all)]
use std::io;

fn main() -> io::Result<()> {
    let mut input_buffer = String::new();
    let mut char_buffer = String::new();
    println!("Input: ");
    io::stdin().read_line(&mut input_buffer).unwrap();
    println!("Char to extract: ");
    io::stdin().read_line(&mut char_buffer).unwrap();
    let trim = trim(&input_buffer, &char_buffer);
    println!("{} {}", trim.0, trim.1);
    Ok(())
}

fn trim(input: &str, char: &str) -> (i32, &'static str) {
    let char = char.chars().last().unwrap();
    println!("Char: {}", char);
    let new_str = String::new();

    return (3, "aaa".clone());
}
