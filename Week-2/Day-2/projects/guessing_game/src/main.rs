use std::io;
use rand::RngExt;

// input a number and it return the number thats all
// takes in input using std::io::stdin()
// variable is immutable by default hence the mut in the variable.
fn main() {
    println!("Welcome to guessing game sanchay ,i will erturn it to you !!");
    let secret_number = rand::rng().random_range(1..=100);
    println!("Guess a number now:");

    let mut guess = 0;

    while guess != secret_number{

        let mut input  = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("the read_line failed");

        guess = input
                   .trim()
                   .parse()
                   .expect("Please enter a number");

        if secret_number > guess {
            println!("your guessed number is lower than the secret_number");
        }
        else if secret_number < guess {
            println!("your guessed number is higher than the secret_number");
        }
        else {
            println!("Boooyahhhh .... you guessed the number {guess} and its right , Let's gooo..");
            break;
        }
    }



}
