// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn print_first_name(first_name: &str) {
    println!("{:?}", first_name)
}

fn print_last_name(last_name: &str) {
    println!("{:?}", last_name)
}

fn main() {
    let my_first_name = "Keith";
    let my_last_name = "Price";

    print_first_name(my_first_name);
    print_last_name(my_last_name)
}
