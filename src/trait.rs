#![deny(clippy::all)]
use std::fmt;

struct Person {
    first_name: String,
    last_name: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Firstname: {}, Lastname: {}",
            self.first_name, self.last_name
        )
    }
}

trait CanHaveFullName {
    fn get_full_name(&self) -> String;
}

impl CanHaveFullName for Person {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait HasFirstAndLastName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

trait HasFullName
where
    Self: HasFirstAndLastName,
{
    fn full_name(&self) -> String;
}

impl<T> HasFullName for T
where
    T: HasFirstAndLastName,
{
    fn full_name(&self) -> String {
        format!("{} {}", self.last_name(), self.first_name())
    }
}

impl HasFirstAndLastName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }
}

trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is runing", self.get_full_name());
    }
}

fn print_full_name(value: &impl CanHaveFullName) {
    println!("{}", value.get_full_name())
}

// fn print_full_name2<T: CanHaveFullName + CanRun>(value: &T) {
//     println!("{}", value.get_full_name());
//     value.run();
// }

fn print_full_name2<T>(value: &T)
where
    T: CanHaveFullName + CanRun,
{
    println!("{}", value.get_full_name());
    value.run();
}

trait HaveFactoryNew {
    fn new(fullname: &str) -> Self;
}

impl HaveFactoryNew for Person {
    fn new(fullname: &str) -> Self {
        let split_name: Vec<&str> = fullname.split(" ").collect();
        Person {
            first_name: split_name[0].to_owned(),
            last_name: split_name[1].to_owned(),
        }
    }
}

fn main() {
    let person = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
    };
    print_full_name(&person);
    let person2 = Person::new("John Mibi");
    print_full_name2(&person2);
    println!("Person2 fullname is: {}", person2.full_name());
}
