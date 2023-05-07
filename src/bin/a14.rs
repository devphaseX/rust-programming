// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

use std::str::FromStr;

fn main() {
    let people = vec![
        Person {
            name: "Daniel".to_owned(),
            favourite_colour: "blue".to_owned(),
            age: 23,
        },
        Person {
            name: String::from("John"),
            favourite_colour: String::from_str("red").unwrap(),
            age: 10,
        },
        Person {
            name: String::from("Micheal"),
            favourite_colour: String::from_str("brown").unwrap(),
            age: 32,
        },
    ];

    for person in &people {
        if person.age <= 10 {
            let Person {
                ref name,
                ref favourite_colour,
                ..
            } = person;

            print_name(name);
            print_colour(favourite_colour);
        }
    }
}

struct Person {
    name: String,
    favourite_colour: String,
    age: u32,
}

fn print_name(name: &str) {
    println!("The name of this person is {}", name);
}

fn print_colour(colour: &str) {
    println!("The favourite colour of this person is {}", colour);
}
