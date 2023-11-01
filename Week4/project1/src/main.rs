use std::io;
fn main() {
 loop{
    let mut tipe = String::new();
    println!("Are you working with metric(m) or imperial(i) values");
    io::stdin().read_line(&mut tipe).expect("Not a valid answer");
    
    if tipe.trim() == "m"{
        let mut distance = String::new();
        let mut time = String::new();

        println!("Please enter your distance in kilometers: ");
        io::stdin().read_line(&mut distance).expect("Invalid format");
        let d:f32 = distance.trim().parse().expect("Invalid format");
        println!("Please enter yout time in hours: ");
        io::stdin().read_line(&mut time).expect("Invalid");
        let t:f32 = time.trim().parse().expect("Invalid");

        let speed = d/t;

        let mut response = String::new();
        println!("Do you wish to represent your answers in metric(m) or imperial(i) form");
        io::stdin().read_line(&mut response).expect("invalid");
        if response.trim() == "m" {
            println!("Your speed is {} kilometers per hour", speed);
        }else{
            println!("Your speed is {} miles per hour", (speed * 0.62137119) );
        }

    }else{
        let mut distance = String::new();
        let mut time = String::new();

        println!("Please enter your distance in miles: ");
        io::stdin().read_line(&mut distance).expect("Invalid format");
        let d:f32 = distance.trim().parse().expect("Invalid format");
        println!("Please enter yout time in hours: ");
        io::stdin().read_line(&mut time).expect("Invalid");
        let t:f32 = time.trim().parse().expect("Invalid");

        let speed = d/t;

        let mut response = String::new();
        println!("Do you wish to represent your answers in metric(m) or imperial(i) form");
        io::stdin().read_line(&mut response).expect("invalid");
        if response.trim() == "m" {
            println!("Your speed is {} kilometers per hour", (speed*1.609344) );
        }else{
            println!("Your speed is {} miles per hour", speed);
        }


    }
    let mut res = String::new();
    println!("Do you wish to continue (y/n)?");
    io::stdin().read_line(&mut res).expect("invalid");

    if res.trim() == "n"{
        break
    }
 }
}
