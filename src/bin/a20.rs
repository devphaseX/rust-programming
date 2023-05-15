// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

#[derive(Debug)]
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn print_power_message(command: Option<PowerState>) {
    match command {
        Some(option) => println!("performing {:?}", option),
        None => {
            println!("Invalid command")
        }
    }
}
fn main() {
    let input = "reboot".to_lowercase();
    let power_type;

    match input.as_str() {
        "off" => {
            power_type = Some(PowerState::Off);
        }
        "sleep" => {
            power_type = Some(PowerState::Sleep);
        }
        "reboot" => {
            power_type = Some(PowerState::Reboot);
        }
        "shutdown" => {
            power_type = Some(PowerState::Shutdown);
        }
        "hibernate" => {
            power_type = Some(PowerState::Hibernate);
        }
        _ => power_type = None,
    }

    print_power_message(power_type)
}
