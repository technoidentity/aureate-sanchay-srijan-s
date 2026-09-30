fn main() {
    let a:u32  = 24;
    let b:i32  = 255;
    let c:u8 = 255;

    let working  = true;
    let sanchay = false;

    if working {
        println!("Someone is working in the office");
    }

    let name = "sanchay";
    let letter:char = '$';

    let youtube_channel = ("sidemen",30000000);

    let (channel_name,subscribers) = youtube_channel;

    println!("The channel name is : {}",channel_name);
    println!("The number of subscribers : {}",subscribers)
}
