#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    fmt,
    io::{self, BufRead, Stdin},
    str,
};

#[derive(Debug)]
enum DrinkSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
enum Flavor {
    Lemonade,
    Orange,
    Grape,
}
// #[derive(Debug)]
struct Drink {
    name: String,
    size: DrinkSize,
    flavor: Flavor,
    price: f64,
}

// enum Option<T> {  // T is a type
//     None,
//     Some(Drink)
// }

impl Drink {
    fn new(name: String, size: DrinkSize, flavor: Flavor, price: f64) -> Self {
        Drink {
            name,
            size,
            flavor,
            price,
        }
    }

    fn print(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Drink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Drink: {}, Size: {:?}, Flavor: {:?}, Price:: {:.2}",
            self.name, self.size, self.flavor, self.price
        )
    }
}

pub fn main() {
    let mut drinks = Vec::new();

    drinks.push(Drink::new(
        "Orange Juise".to_string(),
        DrinkSize::Large,
        Flavor::Orange,
        1.50,
    ));

    drinks.push(Drink::new(
        "Lemonade".to_string(),
        DrinkSize::Medium,
        Flavor::Lemonade,
        2.00,
    ));

    loop {
        println!("Welcome to the drink stand!");
        println!("Here are our drinks:");

        for (i, drink) in drinks.iter().enumerate() {
            println!("{}, {}", i + 1, drink.name);
        }

        let drink_choice: usize; // let x in JS, integer of unknown size

        loop {
            let mut drink_choice_input: String = String::new();
            println!("Enter the number of the drink you want or 'q' to quit");

            io::stdin()
                .read_line(&mut drink_choice_input)
                .expect("Failed to read line");

            if drink_choice_input.trim() == "q" {
                println!("Thank you for visiting the Drink Stand. Have a nice day!");
                return;
            }

            match drink_choice_input.trim().parse::<usize>() {
                Ok(num) => {
                    if num == 0 || num > drinks.len() {
                        println!("Invalid choice. Please enter a valid number.");
                        continue;
                    } else {
                        drink_choice = num;
                        break;
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            }
        }

        // Ask for the size
        let size_choice: DrinkSize;
        loop {
            let mut size_choice_input = String::new();
            println!("Enter the size of the drink you want (Small, Medium, Large):");

            io::stdin()
                .read_line(&mut size_choice_input)
                .expect("Failed to read line");

            match size_choice_input.trim() {
                "Small" => {
                    size_choice = DrinkSize::Small;
                    break;
                }
                "Medium" => {
                    size_choice = DrinkSize::Medium;
                    break;
                }
                "Large" => {
                    size_choice = DrinkSize::Large;
                    break;
                }
                _ => {
                    println!("Invalid size. Please enter a valid size.");
                    continue;
                }
            };
        }

        let chosen_drink = &mut drinks[drink_choice - 1];
        // Update the size of the drink
        chosen_drink.size = size_choice;

        chosen_drink.print();

        println!(
            "You chose a {:?} {}. That'll be ${:.2}. Please enter your payment:",
            chosen_drink.size, chosen_drink.name, chosen_drink.price
        );

        let mut payment = String::new();
        io::stdin()
            .read_line(&mut payment)
            .expect("Failed to read line");

        let payment: f64 = match payment.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid payment. Please enter a valid number.");
                continue;
            }
        };

        if payment < chosen_drink.price {
            println!("Insufficient payment. Please pay full amount.");
            continue;
        }

        println!(
            "Payment successful. Here is your {}. Enjoy!",
            chosen_drink.name
        );
    }

    // let mut drinks = vec![];
}
