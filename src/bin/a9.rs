// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let (_, y) = get_coord();

    if y > 5 {
        println!("The y coord is greater than 5")
    } else if y < 5 {
        println!("The y coord is less than 5")
    } else {
        println!("The y coord is on 5 dot.")
    }
}

fn get_coord() -> (i32, i32) {
    (4, 0)
}
