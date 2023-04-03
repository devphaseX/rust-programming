// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let x = 12;
    let y = 15;
    let result = add(x, y);
    display_result(result);
}

// * Use a function to add two numbers together
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// * Use a function to display the result
fn display_result(i: i32) {
    print!("result: {:?}", i);
}
