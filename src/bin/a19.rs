// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;
fn main() {
    let mut furniture_store = HashMap::new();
    furniture_store.insert("chairs", 5);
    furniture_store.insert("beds", 3);
    furniture_store.insert("table", 2);
    furniture_store.insert("couches", 0);

    for (name, quanitity) in furniture_store.iter() {
        if quanitity == &0 {
            println!("{} is out of stock", name);
            continue;
        }
        println!("found {} quantities for the item {}", quanitity, name);
    }

    println!("{:?}", furniture_store);
}
