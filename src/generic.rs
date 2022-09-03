#![deny(clippy::all)]

use std::ops::AddAssign;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: AddAssign,
{
    fn move_offset(&mut self, move_x: T, move_y: T) {
        self.x += move_x;
        self.y += move_y;
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, move_offset: Self) {
        self.x += move_offset.x;
        self.y += move_offset.y;
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 3.0 };
    let mut p2 = Point { x: 0.0, y: 3.3 };
    p2.move_offset(3.0, 5.0);
    println!("{:?}", p2);
    p2 += p1;
    println!("{:?}", p2);
    let p3 = Point { x: 3.0, y: 11.3 };
    println!("{}", p2 == p3);
}
