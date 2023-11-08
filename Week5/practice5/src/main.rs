use std::io;
fn main() {
    let mut input = String::new();

    println!("\nEnter Your height(in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height: f32= input.trim().parse().expect("Not a valid integer");

    if height >=160. && height <=170.0{
        println!("You are of Average height");

    }else if height > 170.0 && height <= 195.0{
        println!("You are tall");
    }else if height < 160.0 && height > 100.{
        println!("You are a dwarf");
    }else{
        println!("Abnormal height");
    }
}
