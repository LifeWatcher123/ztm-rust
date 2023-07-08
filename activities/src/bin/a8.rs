// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinkFlavor {
    Lemon,
    Orange,
    SodaPop,
    Mango,
}

struct Drink {
    flavor: DrinkFlavor,
    fl_oz: f64,
}

fn print_drink_info (drink: Drink) {
    match drink.flavor {
        DrinkFlavor::Lemon => println!("Flavor: {:?}", "lemon"),
        DrinkFlavor::Mango => println!("Flavor: {:?}", "mango"),
        DrinkFlavor::Orange => println!("Flavor: {:?}", "orange"),
        DrinkFlavor::SodaPop => println!("Flavor: {:?}", "soda"),
    }
    println!("Fluid Ounce: {:?}", drink.fl_oz);
}

fn main() {
    let x = Drink {
        flavor: DrinkFlavor::Mango,
        fl_oz: 10.00
    };

    print_drink_info(x);
}
