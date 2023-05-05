// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

fn main() {
    let item = GroceryItem {
        id: 2,
        quantity: 107,
    };

    display_id(&item);
    display_quantity(&item);
}

struct GroceryItem {
    id: u32,
    quantity: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("The quantitiy for the grocery item is {:}", item.quantity);
}
fn display_id(item: &GroceryItem) {
    println!("The id for the grocery item is {:}", item.id);
}
