///////////////////////////////////////////
// BAI 1
// Yêu cầu :
// + Sửa code liên quan tới vấn đề generic type (thay đổi ở định nghĩa struct)
///////////////////////////////////////////

// struct Point<T, R> {
//     x: T,
//     y: R,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point {
//         x: 5,
//         y: "hello".to_string(),
//     };

//     println!("Success!");
// }

///////////////////////////////////////////
// BAI 2
// Yêu cầu :
// + Implement hàm sum dưới đây, sao cho việc kiểm tra assert_eq đúng
///////////////////////////////////////////

// use std::any::TypeId;
// use std::ops::Add;

// // Implement the generic function below.
// fn sum<T: std::ops::Add + Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }

// pub const fn is_signed<T: ?Sized>() -> bool {
//     true
// }

///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng
///////////////////////////////////////////

// fn main() {
//     let a = A {
//         p: Some("p".to_string()),
//     };
//     a.a();
// }

// struct A {
//     p: Option<String>,
// }

// impl A {
//     fn a(self) -> Self {
//         Self::b(&self.p.unwrap());
//         self
//     }
//     fn b(b: &str) {
//         print!("b: {}", b)
//     }
// }

///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng
///////////////////////////////////////////

#[derive(Debug, Clone)]
struct MyData {
    val1: i32,
    val2: String,
}

fn main() {
    let d = MyData {
        val1: 35,
        val2: String::from("Hello World"),
    };

    let both = d.get_both();
    let x = d.get_val1();
    let y = d.get_val2();
}

impl MyData {
    pub fn get_val1(&self) -> &i32 {
        return &self.val1;
    }

    pub fn get_val2(&self) -> &String {
        return &self.val2;
    }

    pub fn get_both(&self) -> (&i32, &String) {
        return (&self.val1, &self.val2);
    }
}
