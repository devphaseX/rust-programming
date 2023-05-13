// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

fn main() {
    let student = Student {
        name: "Ayomide".to_owned(),
        locker: Some(13),
    };

    println!("student {:?}", student.name);
    match student.locker {
        Some(locker_number) => println!("locker number: {:?}", locker_number),
        _ => println!("no locker number"),
    }
}

struct Student {
    name: String,
    locker: Option<i32>,
}
