use std::io;
fn main() {
    let mut age = String::new();
    let mut response = String::new();

    println!("How old are you: ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:i32 = age.trim().parse().expect("Not a valid Number");
    println!("Are you experienced (y/n)");
    io::stdin().read_line(&mut response).expect("Not a valid response");

    loop{
        if response.trim() == "n"{
            break;
        }else if response.trim() == "y"{
            break;
        }else{
            println!("You entered an invalid answer. please use y or n to show if you are experienced or not");
            io::stdin().read_line(&mut response).expect("Not a valid response");
            
        }

    }
    if response.trim() == "y" && age >= 40{
        println!("Their incentive is 1560000");
    }else if response.trim() == "y" && age >=30 && age < 40{
        println!("Their incentive is 1480000");
    }else if response.trim() == "y" && age < 30{
        println!("Their incentive is 1300000");
    }else if response.trim() == "n"{
        println!("Their incentive is 100000");
    }

}
