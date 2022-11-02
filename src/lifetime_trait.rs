/////////////////////////////////////
// Bài 1
// Sửa code để compile thành công liên quan tới vấn đề lifetime
/////////////////////////////////////

use std::io;
fn main() {
    let mut input: Vec<String>;
    loop {
        let mut input_text = String::new();
        println!("Type instruction in the format Add <name> to <department>:");
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        let trimmed_text: String = input_text.trim().to_string();
        input = trimmed_text.split(" ").map(String::from).collect();
        if input.len() < 3 {
            println!("Invalid format.");
        } else {
            if input[0] == "Add" && input[2] == "to" {
                break;
            } else {
                println!("Invalid format.");
            }
        }
    }
    println!("{:?}", input);
}

/////////////////////////////////////
// Bài 2
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        self + "Bar"
    }
}

fn main() {
    let s: String = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}

/////////////////////////////////////
// Bài 3
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_owned());
        self
    }
}

fn main() {}

//TODO: Add your code here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

/////////////////////////////////////
// Bài 4
// Implement trait
// Thực hiên cargo test
/////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// The trait
////////////////////////////////////////////////////////////////////////////////

trait Price {
    fn price(&self, item_name: &str) -> Option<f32>;

    fn total_price(&self, shopping_list: &[&str]) -> Option<f32> {
        // Goal: compute the total price of all items in the shopping
        // list. If any of the options are not present, return `None`.
        if shopping_list.is_empty() {
            None
        } else {
            shopping_list
                .iter()
                .map(|item_name| self.price(item_name))
                .fold(Some(0.0), |acc, price_option| {
                    // None
                    if price_option.is_none() {
                        None
                    } else {
                        Some(acc.unwrap() + price_option.unwrap())
                    }
                })
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Store
////////////////////////////////////////////////////////////////////////////////

struct Store {
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    name: &'static str,
    price: f32,
}

impl Store {
    fn new() -> Store {
        Store { items: vec![] }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

impl Price for Store {
    fn price(&self, item_name: &str) -> Option<f32> {
        for item in &self.items {
            if item.name == item_name {
                return Some(item.price);
            }
        }
        None
    }
}

fn build_store() -> Store {
    let mut store = Store::new();
    store.add_item(Item {
        name: "chocolate",
        price: 5.0,
    });
    store.add_item(Item {
        name: "socks",
        price: 23.0,
    });
    store.add_item(Item {
        name: "plush Mozilla dinosaur",
        price: 13.0,
    });
    store
}

////////////////////////////////////////////////////////////////////////////////
// Factory
////////////////////////////////////////////////////////////////////////////////

// A factory for just a single kind of item
struct Factory {
    item_name: &'static str,
    wholesale_price: f32,
}

impl Price for Factory {
    fn price(&self, item_name: &str) -> Option<f32> {
        if self.item_name == item_name {
            Some(self.wholesale_price)
        } else {
            None
        }
    }
}

fn build_factory() -> Factory {
    Factory {
        item_name: "sprocket",
        wholesale_price: 7.67,
    }
}

fn main() {
    build_store();
    build_factory();
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[test]
fn total_price_store() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur"];
    assert_eq!(store.total_price(&list), Some(18.0));
}

#[test]
fn total_price_missing_store() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur", "fork and knife"];
    assert_eq!(store.total_price(&list), None);
}

#[test]
fn total_price_factory() {
    let factory = build_factory();
    let list = vec!["sprocket"];
    assert_eq!(factory.total_price(&list), Some(7.67));
}

#[test]
fn total_price_missing_factory() {
    let factory = build_factory();
    let list = vec!["sprocket", "socks"];
    assert_eq!(factory.total_price(&list), None);
}

#[test]
fn total_price_empty_store() {
    let store = build_store();
    let list = vec![];
    assert_eq!(store.total_price(&list), None);
}
