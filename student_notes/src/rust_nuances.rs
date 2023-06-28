#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
// ownership, borrowing, lifetimes, references, borrow checker, and pointers

fn bf1(s: &String) {
    println!("{s}"); // new pointer because of reference, the &
}

fn bf2(s: String) {
    println!("{s}");
}

fn borrow_func() {
    let string1 = "Hello".to_string();
    bf1(&string1);
    println!("=================================");
    bf2(string1.clone());
}

// fn move_func() {
//     let x: i32 = 1;
//     let y: i32 = x;
//     println!("{x}, {y}")
// }

fn move_func() {
    let str1: String = "Hello".to_string();
    let str2: String = str1.clone();
    println!("{str1}, {str2}")
}

fn expr_func() {
    let x = 1;

    // loop is like a "while true" loop}
}

fn main() {
    borrow_func();
    move_func();
}
