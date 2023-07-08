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
    id_num: i32,
    quantity: i32,
}

fn display_quantity(grocery_item: &GroceryItem) {
    println!("{:?}", grocery_item.quantity)
}

fn display_id_number(grocery_item: &GroceryItem) {
    println!("{:?}", grocery_item.id_num)
}

fn main() {
    let x = GroceryItem {
        id_num: 123,
        quantity: 12,
    };

    display_id_number(&x);
    display_quantity(&x);
}
