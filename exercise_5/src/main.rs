// Bài tập cho trait
// Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct sau

// #[derive(Debug)]
// struct Fibonacci {
//     a: u32,
//     b: u32,
// }

// impl Iterator for Fibonacci {
//     type Item = u32;


//     // Trả về số fibonaci tiếp theo dựa trên kiểu dữ liệu struct Fibonacci
//     fn next(&mut self) -> Option<u32> {
//         let t = self.b;
//         self.b = self.a;
//         self.a = self.a + t;
        
//         Some(self.a)
//     }
// }

// // Khởi tạo ban đầu cho Fibonaci: 0, 1
// fn fibonacci_numbers() -> Fibonacci {
//     Fibonacci { a: 1, b: 0 }
// }

// fn main() {
//     for number in fibonacci_numbers() {
//         println!("{}", number);
//     }
// }




// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

use std::fmt;
struct StrDisplayable<'a>(Vec<&'a str>);

impl fmt::Display for StrDisplayable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_foo = StrDisplayable(vec);
        println!("{}",vec_foo);
}