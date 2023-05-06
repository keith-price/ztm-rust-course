// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

#[derive(Debug)]
enum Color {
    Blue,
    Red,
    Green
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("{:?}", color),
        Color::Red => println!("{:?}", color),
        Color::Green => println!("{:?}", color)
    }
}

fn main() {
    let my_color = Color::Green;
    
    print_color(my_color)
}
