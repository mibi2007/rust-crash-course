#![deny(clippy::all)]

fn get_values<'a>(values: (&'a str, &'a str, i32)) -> (&str, i32) {
    (values.1, values.2)
}

fn main() {
    let values = ("asd", "dsa", 3i32);
    println!("{}", values.0);
    println!("{}", values.1);
    println!("{}", values.2);
    let (a, b, c) = values;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    let (d, e) = get_values(values);
    println!("{}", d);
    println!("{}", e);
    println!("{}", values.0);
}
