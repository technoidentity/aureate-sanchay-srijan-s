use std::io;

fn quick_sort<T:Ord + Clone>(arr:&mut Vec<T>)-> Vec<T>{
    let len = arr.len();

    if len <= 1 {
        return arr.to_vec();
    }

    let pivot = arr[0].clone();

    let mut smaller = Vec::new();
    let mut larger = Vec::new();    

    for value in &arr[1..]{
        if *value < pivot{
            smaller.push(value.clone());
        }
        else{
            larger.push(value.clone());
        }
    }

    let mut sorted = quick_sort(&mut smaller);
    sorted.push(pivot);
    sorted.extend(quick_sort(&mut larger));

    sorted

}

fn main(){
    println!("Enter the elements of the array separated by spaces: ");
    let mut input_arr = String::new();


    io::stdin()
        .read_line(&mut input_arr)
        .expect("Failed to read line");

    let mut input_arr = input_arr.trim().split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    println!("The sorted array is {:?}: ",quick_sort(&mut input_arr));


}