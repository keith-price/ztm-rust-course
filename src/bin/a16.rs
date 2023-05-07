// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker {
    name: String,
    locker_num: Option<i32>,
}

fn main() {
    let locker_1 = Locker {
        name: "Gerbus Toorflod".to_owned(),
        locker_num: None,
    };

    match locker_1.locker_num {
        Some(num) => println!("{:?}", num),
        None => println!("No locker number was assigned"),
    }
}
