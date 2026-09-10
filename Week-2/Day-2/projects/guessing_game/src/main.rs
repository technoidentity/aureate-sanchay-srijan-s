use std::io;
// input a number and it return the number thats all
// takes in input using std::io::stdin()
// variable is immutable by default hence the mut in the variable.
fn main() {
    println!("Welcome to guessing game sanchay ,i will erturn it to you !!");
    println!("Guess a number now,:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("the read_line failed");

    println!("you guessed the number {guess}");
}
