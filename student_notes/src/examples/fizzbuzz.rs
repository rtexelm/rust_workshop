// let num = 15;

pub fn main() {
    fn fizz_buzz(num: u32) {
        for i in 1..=num {
            if (i % 3 == 0) && (i % 5 == 0) {
                println!("fizzbuzz")
            } else if i % 3 == 0 {
                println!("fizz")
            } else if i % 5 == 0 {
                println!("buzz")
            } else {
                println!("{i}")
            }
        }
    }
    let num = 15;
    fizz_buzz(num)
}
