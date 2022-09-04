#![deny(clippy::all)]

fn main() {
    let name1 = "Mibi";
    let name2 = name1;
    println!("Hello {:p}", name1);
    println!("Hello {:p}", name2);
}
