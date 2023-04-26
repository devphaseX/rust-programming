// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#![allow(dead_code)]
fn main() {
    let drink = Drink {
        flavor: DrinkFlavor::Berry("lemonade".to_owned()),
        fluid_ounces: 32f32,
    };

    print_drink(&drink);
}

#[derive(Debug)]
enum DrinkFlavor {
    Citrus(String),
    Berry(String),
    Tropical(String),
    Spicy(String),
    Minty(String),
}

struct Drink {
    flavor: DrinkFlavor,
    fluid_ounces: f32,
}

fn print_drink(drink: &Drink) {
    match drink.flavor {
        DrinkFlavor::Berry(ref kind) => {
            println!("berry: {:?} with the quanity {:}", kind, drink.fluid_ounces)
        }

        DrinkFlavor::Citrus(ref kind) => {
            println!(
                "citrus: {:?} with the quanity {:}",
                kind, drink.fluid_ounces
            )
        }

        DrinkFlavor::Spicy(ref kind) => {
            println!("spicy: {:?} with the quanity {:}", kind, drink.fluid_ounces)
        }

        DrinkFlavor::Minty(ref kind) => {
            println!("minty: {:?} with the quanity {:}", kind, drink.fluid_ounces)
        }

        DrinkFlavor::Tropical(ref kind) => {
            println!(
                "tropical {:?} with the quanity {:}",
                kind, drink.fluid_ounces
            )
        }
    }
}
