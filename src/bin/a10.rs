// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn red_alert(status: bool) {
    match status {
        true => println!("Explosion is iminent"),
        false => println!("Current levels are within safe zone"),
    };
}

fn main() {
    let current_reading = 69;
    let will_explode = current_reading > 100;

    red_alert(will_explode)
}
