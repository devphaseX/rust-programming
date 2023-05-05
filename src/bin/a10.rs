// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let my_number = 101;
    let greater_than_100 = my_number > 100;
    print_result(greater_than_100);
}

fn print_result(it_big: bool) {
    match it_big {
        true => {
            println!("It big")
        }
        _ => {
            println!("It small")
        }
    }
}
