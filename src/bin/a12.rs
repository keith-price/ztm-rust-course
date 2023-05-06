// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox {
    dimensions: (f64, f64),
    weight: f64,
    color: Color,
}

enum Color {
    Blue,
    Green,
    Lail
}

impl ShippingBox {
    fn new_box(dimensions: (f64, f64), weight: f64, color: Color) -> Self {
        ShippingBox { dimensions: dimensions, weight: weight, color: color }
    }
    
    fn print_box_details(&self) {
        println!("Dimensions: {:?}\nWeight: {:?}", self.dimensions, self.weight);
        match self.color {
            Color::Blue => println!("Blue"),
            Color::Green => println!("Green"),
            Color::Lail => println!("Lail")
        };
    }
}


    

fn main() {
   let my_box = ShippingBox::new_box((32.7, 67.9), 67.34, Color::Green);
    
    ShippingBox::print_box_details(&my_box)
}
