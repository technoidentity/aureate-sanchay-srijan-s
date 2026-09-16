use std::io;
use std::cmp::Ordering;
use rand::RngExt;
use colored::*;


fn main(){
    println!("Welcome to the guessing game sanchay");

    println!("Guess a number");


    let secret_number  = rand::rng().random_range(1..101);

    loop{

        let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line pls try again");

    // println!("You guessed the number:  {}",guess);

    let guess : u32 = guess.trim().parse().expect("Please enter a number");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("{}","you guesss number is less".red()),
        Ordering::Greater => println!("{}","your guessed number was larger".red()),
        Ordering::Equal => {
            println!("{}","You have guessed it right ...  Well done".green());
            break;
        }
    }

    }
}
