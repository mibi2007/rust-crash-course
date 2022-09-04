#![deny(clippy::all)]

fn main() {
    // let name = Some("John Doe");
    let name = None;
    // let unwrap = name.unwrap_or("Default");
    let unwrap = name.unwrap_or_else(|| {
        println!("Will fallback to Default");
        "Default"
    });
    println!("{}", unwrap);
}
