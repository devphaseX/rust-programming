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
    let first_name = "Ayomide";
    let last_name = "Lawal";
    display_name(&first_name);
    display_name(&last_name);
}

fn display_name(name: &str) {
    println!("{}", name);
}
