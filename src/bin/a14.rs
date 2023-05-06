// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

impl Person {
    fn peep_details(&self) {
        println!(
            "Name: {:?} -- Favorite Color: {:?}",
            self.name, self.fav_color
        )
    }
}

fn main() {
    let peeps_vec = vec![
        Person {
            age: 3,
            name: String::from("Krol Dingon"),
            fav_color: String::from("Lant"),
        },
        Person {
            age: 34,
            name: String::from("Sifon Pertsek"),
            fav_color: String::from("Rooje"),
        },
        Person {
            age: 7,
            name: String::from("Drapoi Geqwel"),
            fav_color: String::from("Crerl"),
        },
    ];

    for peep in peeps_vec {
        match peep.age {
            0..=10 => peep.peep_details(),
            _ => (),
        }
    }
}
