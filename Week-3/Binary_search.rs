use std::io;

fn binary_search(target:i32,arr:&Vec<i32>)->bool{
    let mut l = 0;
    let mut h = arr.len();

    while l<h{
        let mid = l + (h-l)/2;

        if arr[mid] == target{
            return true;
        }
        else if arr[mid] < target{
            l = mid + 1;
        }
        else{
            h = mid;
        }
    }
    return false;
}


fn main(){
    println!("Enter the elements of the array separated by spaces: ");
    let mut input_arr = String::new();

    io::stdin()
        .read_line(&mut input_arr)
        .expect("Failed to read line");

    let input_arr = input_arr.trim().split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    println!("Enter the target element: ");
    let mut target_input = String::new();
    io::stdin()
        .read_line(&mut target_input)
        .expect("Failed to read line");
    let target: i32 = target_input.trim().parse().expect("Please enter a valid integer");

    println!("the target element search was found : {}",binary_search(target,&input_arr));
}