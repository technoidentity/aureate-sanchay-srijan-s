use std::io;

fn main() {
    println!("Welcome to guessing game");
    println!("Hey buddy ,Guess a number ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("the read_line failed");

    println!("you guessed the number {guess}");
}
