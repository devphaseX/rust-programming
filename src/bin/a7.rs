// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

#![allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
fn main() {
    let color = Color::Red;
    print_color(&color);
    println!("{:?}", color);
}

fn print_color(color: &Color) {
    match color {
        Color::Blue => {
            println!("na blue")
        }

        Color::Green => {
            println!("na green")
        }

        Color::Red => {
            println!("na red")
        }
    }
}
