// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add_two_nums(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn print_sum(sum: i32) {
    println!("{:?}", sum)
}

fn main() {
    let first_num = 120;
    let second_num = 6;

    let total = add_two_nums(first_num, second_num);

    print_sum(total)
}
