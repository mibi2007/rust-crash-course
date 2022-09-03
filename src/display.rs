#![deny(clippy::all)]

use std::{fmt, ops::AddAssign};

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

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point(x: {:?}, y: {})", self.x, self.y,)
    }
}

impl<T> fmt::Debug for Point<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {:?})", self.x, self.y,)
    }
}

fn main() {
    // let mut p1 = Point { x: 0, y: 3 };
    let mut p2 = Point { x: 0.0, y: 3.3 };
    p2.move_offset(3.0, 5.0);
    println!("{}", p2);
    println!("{:?}", p2);
}
