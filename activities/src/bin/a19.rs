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
    let mut furniture_stock: HashMap<_, i32> = HashMap::new();

    furniture_stock.insert("Chair".to_string(), 5);
    furniture_stock.insert("Bed".to_string(), 3);
    furniture_stock.insert("Table".to_string(), 2);
    furniture_stock.insert("Couch".to_string(), 0);

    let mut total_stock = 0;

    // Apparently, you need to use reference here when comparing with an if block. Should research.
    for (furniture, stock) in &furniture_stock {
        // Alternatively, it seems that * helps. Should research.
        // Crash course used a macro named `format!` to store a variable to a string literal??
        if *stock == 0 {
            println!("Furniture: {:?}, Stock: <out of stock>", furniture);
        } else {
            println!("Furniture: {:?}, Stock: {:?}", furniture, stock);
        };

        total_stock += stock;
    }

    println!("\nTotal Stock: {:?}", total_stock);
}
