use std::io;

fn insertion_sort(arr:&mut Vec<i32>)->Vec<i32>{
    let len = arr.len();
    let mut sorted_arr = arr.clone();

    for i in 1..len{
        let key = sorted_arr[i];
        let mut j = i;

        while j>0 && sorted_arr[j-1] > key{
            sorted_arr[j] = sorted_arr[j-1];
            j-=1;
        }
        sorted_arr[j] = key;
    }

    sorted_arr
}


fn main() {
    println!("Enter the elements of the array separated by spaces: ");

    let mut input_arr = String::new();

    io::stdin()
        .read_line(&mut input_arr)
        .expect("Failed to read line");

    let mut input_arr = input_arr.trim().split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let sorted_arr = insertion_sort(&mut input_arr);

    println!("The sorted array is : {:?}",sorted_arr);
}