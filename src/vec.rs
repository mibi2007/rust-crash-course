#![deny(clippy::all)]

fn main() {
    let mut values = vec![2, 5];
    let valuea2 = vec![4, 6];
    values.extend(valuea2);
    // values.push(7);
    println!("{:?}", values);
    // values.clear();
    // values.extend_from_slice(&[5, 6, 7]);
    // let mut a = values.pop();
    // let b = a.get_or_insert(-1);
    // println!("{:?}", values);
    // println!("{:?}", b);
}
