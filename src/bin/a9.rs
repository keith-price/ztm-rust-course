// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coords() -> (i32, i32) {
    (2, 0)
}

fn main() {
    // _x as x is not bieng used in this function
    let (_x, y) = coords();
    if y > 5 {
        println!(">5")
    } else if y < 5 {
        println!("<5")
    } else {
        println!("=5")
    }
}
