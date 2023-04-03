// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    display_name();
}

fn display_name() {
    let first_name = "Ayomide";
    let last_name = "Lawal";

    println!("{} {}", first_name, last_name);
}
