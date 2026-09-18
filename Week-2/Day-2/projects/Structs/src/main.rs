#[derive(Debug)]
struct User {
     Name:String,
     Age:u32,
     Height:u32,
     Weight:u32,
     Country:String,
     Language:String,
}

fn main() {
    let Aadhar = User{
        Name:String::from("Sanchay"),
        Age:23,
        Height:181,
        Weight:76,
        Country:String::from("India"),
        Language:String::from("Tamil"),
    };
    let Aadhar_info = Citizen(Aadhar);
    println!("Hello , Your aadhar info is {:?}",Aadhar_info);
}

fn Citizen(Aadhar:User)->User {
    Aadhar
}
