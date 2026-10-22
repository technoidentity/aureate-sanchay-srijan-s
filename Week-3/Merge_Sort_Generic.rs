use std::io;

fn merge_sort<T:Ord+Clone>(arr:&mut Vec<T>){
    let len  = arr.len();
    if len == 1{
        return ;
    }

    let mid = len/2;

    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr,left,right);
}

fn merge<T:Ord+Clone>(arr:&mut Vec<T>,left:Vec<T>,right:Vec<T>){
    let mut i = 0;
    let mut j = 0;          
    let mut k = 0;

    while i < left.len() && j<right.len(){
        if left[i] <= right[j]{
            arr[k] = left[i].clone();
            i+=1;
        }
        else{
            arr[k] = right[j].clone();
            j+=1
        }
        k+=1;
    }

    while i < left.len(){
        arr[k] = left[i].clone();
        i+=1;
        k+=1;
    }   

    while j < right.len(){
        arr[k] = right[j].clone();
        j+=1;
        k+=1;
    }
}



fn main() {
    println!("Enter the elements of the array separated by spaces: ");

    let mut input_arr = String::new();

    io::stdin()
        .read_line(&mut input_arr)
        .expect("Failed to read line");

    let mut input_arr = input_arr
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    merge_sort(&mut input_arr);

    println!("Sorted array: {:?}", input_arr);

    
}