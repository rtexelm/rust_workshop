#![allow(dead_code)]

/*
Structs in Rust:

Rust has three types of structures "structs" that can be created for use in a program:
named-field structs, tuple structs, and unit structs. They differ in how their fields are accessed.
*/

/*
Named-field structs:

    This is the most common type of struct and consists of an optional visibility, a name,
    and named fields. Each field has a type, and its name is used to access the data it contains.
*/

/*
Tuple structs:

    These are structs that have fields with types but no names. These fields can be accessed by
    using dot notation along with the index of the field (like tuples).
*/

/*
Unit structs:

    These are structs without any fields. They are used primarily for generic programming.
    While at first glance they might seem useless (after all, they don't actually store any data),
    unit structs can actually be quite useful in certain scenarios.

    Marker or Phantom types: They can be used to create distinct types for type safety and clarity,
    even when they don't need to carry data. For example, you might have two unit structs, Inches and Centimeters,
    that you use as types for function parameters to make sure you don't mix them up.

    Traits and trait objects: If you have a trait that doesn't need any data, you can implement it for a unit
    struct and then use trait objects of that type.
*/

pub fn main() {
    #[derive(Debug)]
    struct Pet {
        name: String,
        pet_type: String,
    }

    #[derive(Debug)]
    struct Human {
        pub name: String,
        age: i32,
        pets: Vec<Pet>,
    }

    impl Human {
        fn new(name: String, age: i32, pets: Vec<Pet>) -> Self {
            Self { name, age, pets }
        }

        // Println would consume the variable so add the following line otherwise it destroys the pointer after being called
        pub fn print(&self) {
            println! {"Ther person's name is: {}, and age is: {}", self.name, self.age}
        }
    }

    let baby: Pet = Pet {
        name: "baby".to_string(),
        pet_type: "chihuahua".to_owned(),
    };

    let human: Human = Human {
        name: String::from("Sir Mittens Catington III"),
        age: 42,
        pets: vec![baby],
    };

    let matilda: Human = Human::new(
        "matilda cubbins".to_string(),
        5,
        vec![Pet {
            name: "Lisa".to_string(),
            pet_type: "beta fish".to_string(),
        }],
    );
}
