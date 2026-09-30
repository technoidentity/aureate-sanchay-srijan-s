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
    let mut Aadhar = User{
        Name:String::from("Sanchay"),
        Age:23,
        Height:181,
        Weight:76,
        Country:String::from("India"),
        Language:String::from("Tamil"),
    };
    let Aadhar_info = &Aadhar;
    Citizen(Aadhar_info);
    Language(Aadhar_info);
    println!("Thank you {}",&Aadhar_info.Name);
}

fn Citizen(Aadhar:&User) {
    println!("HI {}",Aadhar.Name);
    println!("You are a citizen of :{}",Aadhar.Country);
}
fn Language(Aadhar:&User) {
    println!("You Speak the language of :{}",Aadhar.Language);
}
