use std::io;

fn selection_sort<T:Ord +Clone>(arr:&mut Vec<T>)->Vec<T>{
    let len = arr.len();
    let mut sorted_arr = arr.clone();

    for i in 0..len{
        let mut min_index = i;

        for j in i+1..len{
            if sorted_arr[j] < sorted_arr[min_index]{
                min_index = j;
            }
        }
        sorted_arr.swap(i,min_index);
    }
    sorted_arr
}

fn main(){
    println!("Enter the elements of the array to be sorted separated by spaces: ");

    let mut input_arr = String::new();

    io::stdin()
        .read_line(&mut input_arr)
        .expect("Failed to read line");

    let mut input_arr = input_arr.trim().split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let sorted_array = selection_sort(&mut input_arr);

    println!("The sorted array is : {:?}",sorted_array);

}