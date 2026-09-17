// For this lecture i was able to understand the use of functions and loop.
// I thought i could use it for a own idea
// Fibonacci series.

use std::io;
use colored::*;


fn main() {
    println!("Fibonacci series");
loop{
    println!("Please enter the number of elements you want with the fibonacci series :");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Error:Cannot readline ");
    let n:usize = n.trim().parse().expect("Please enter a number");

    if n<=2{
        println!("{}","Warning:Fibonacci series needs atlest 2 initial numbers".red())
    }

    else{

    let Fibonacci_series = Fibonacci_series(n);
    println!("The Fibonacci_series is: {:?}",Fibonacci_series);
    break;
    }
}

}

// Important  : we use usize as an unsigned integer type when we are working or comparing with sizes/indexes ,
// same goes for the n above and below for defining the input of the function.


fn Fibonacci_series(n:usize)-> Vec<i32>{
    let mut series = vec![1,1];
    // vec![] for initializing with something
    // Vector::new(); for a new empty string

    while series.len() < n {
        // use series.len() to get the size of the vector

            let x = series[series.len()-1] + series[series.len() -2];
            series.push(x);

    }
    series
}
