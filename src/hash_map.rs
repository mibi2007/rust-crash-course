#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();

    map.entry("poneyland")
        .and_modify(|e| *e = *e + 1)
        .or_insert(42);
    assert_eq!(map["poneyland"], 42);
    println!("{:?}", map);

    map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
    assert_eq!(map["poneyland"], 43);
    println!("{:?}", map);

    let entry = map.entry("poneyland");

    match entry {
        std::collections::hash_map::Entry::Occupied(entry) => {
            println!("{}", entry.get())
        }
        std::collections::hash_map::Entry::Vacant(empty) => {
            println!("{:?}", empty)
        }
    }
}
