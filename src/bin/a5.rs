// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut my_int = 0;
    
    loop {
        while my_int <=4 {
            println!("{:?}", my_int);
            my_int += 1
        }
        break
    }
}
