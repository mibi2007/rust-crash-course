#![deny(clippy::all)]

use intutils::addition::add;

fn main() {
    let value = add(1, 2);
    println!("Value: {}", value)
}
