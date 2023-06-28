// ownership, borrowing, lifetimes, references, borrow checker, and pointers

fn bf1(s: &String) {
    println!("{s}");
}

fn borrow_func() {
    let string1 = "Hello".to_string();
}

fn main() {
    borrow_func();
}
