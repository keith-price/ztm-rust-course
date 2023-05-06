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

enum Flavor {
    Orange,
    Apple,
    Lime
}
struct Drink {
    flavor: Flavor,
    floz: f64
}

fn print_drink_details(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("Flavor: Orange"),
        Flavor::Apple => println!("Flavor: Apple"),
        Flavor::Lime => println!("Flavor: Lime")
    }
    println!("Fluid ounces: {:?}", drink.floz)
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Lime,
        floz: 36.99,
    };
    
    print_drink_details(my_drink)
        
}
