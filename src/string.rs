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
    println!("Identical count {}", trim.0);
    println!("New string {}", trim.1);
    Ok(())
}

fn trim(input: &str, char: &str) -> (i32, String) {
    let char = char.chars().nth(0).unwrap();
    println!("Char: {}", char);
    let mut new_str = String::new();
    let mut identical_count = 0;
    for input_char in input.chars() {
        if input_char == char || input_char == char.to_uppercase().nth(0).unwrap() {
            identical_count += 1;
        } else {
            new_str.push(input_char)
        }
    }

    return (identical_count, new_str);
}
