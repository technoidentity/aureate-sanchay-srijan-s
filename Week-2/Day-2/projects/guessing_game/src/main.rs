use std::io;
use std::cmp::Ordering;
use rand::RngExt;
use colored::*;

// sanchay's guessing game
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


// the below is where i worked on data types and functions and loops
// i was able to understand multiple data types ,
// in rust when there is a integer overflow , it will reset to 1 , like if i save a integer as 256 in the u8 that is the unsigned 8-bit integer/
