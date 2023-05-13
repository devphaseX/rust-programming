// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Bill".to_owned()),
        Ticket::Vip(30.0, "Micheal".to_owned()),
        Ticket::Standard(10.0),
    ];

    for ticket in &tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage holder: {:?}, price: {:?}", price, holder)
            }
            Ticket::Standard(price) => println!("Standard holder: {:?}", price),
            Ticket::Vip(price, holder) => println!("Vip holder: {:?}, price: {:?}", price, holder),
        }
    }
}

enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}
