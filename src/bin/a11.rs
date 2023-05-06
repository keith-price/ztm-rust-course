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

struct GroceryItem {
    name: String,
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!(
        "The remaning quanity of {:?} is {:?}",
        item.name, item.quantity
    )
}

fn display_id(item: &GroceryItem) {
    println!("The id of {:?} is {:?}", item.name, item.id)
}

fn main() {
    let lettuce = GroceryItem {
        name: String::from("Lettuce"),
        quantity: 2,
        id: 67,
    };

    display_quantity(&lettuce);
    display_id(&lettuce);
}
