#![deny(clippy::all)]

use std::ops::Deref;

struct BoxValue<T> {
    value: T,
}

impl<T> BoxValue<T> {
    fn new(value: T) -> BoxValue<T> {
        BoxValue { value }
    }
}

impl<T> Deref for BoxValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let age = BoxValue::new(30);
    let ref_age = age.deref();
    let original_age = *(age.deref());
    let double_age = ref_age * 2;
    println!("{}", double_age);
    println!("{}", "double_age");
    println!("{}", original_age);
    println!("{}", "original_age");
    println!("{}", age.value);
}
