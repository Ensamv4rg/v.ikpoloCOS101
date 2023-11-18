use std::io;
fn main() {
    println!("The menu is as follows");
    println!("Poundo Yam and Edinkaiko Soup for N3200");
    println!("Fried Rice & Chicken for N3000");
    println!("Amala & Ewedu Soup for N2500");
    println!("Eba & Egusi Soup for N2000");
    println!("White Rice & Stew for N2500");

    println!("What would you like to eat?");
    println!("Use P for Poundo, F for Fried Rice, A for Amala, E for Eba, W for white rice");
    println!("Type D when you're done");
    
    let mut cost:i32 = 0;
    loop{
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Not a valid answer");

        if choice.trim() == "P"{
            cost += 3200;


        }else if choice.trim() == "F"{
            cost += 3000;
        }else if choice.trim() == "A" {
            cost += 2500;
        }else if choice.trim() == "W"{
            cost += 2500;
        }else if choice.trim() == "D"{
            break;
        }else if choice.trim() == "E" {
            cost += 2000;

        }else{
            println!("Note, you typed an invalid answer therefore, it wasn't recorded. Please check ");
            continue;
        }
    }
    if cost > 10000{
        println!("Beacuse your goods are greater than 10000, you qualified for a 5% discount");
        let cost:f32 = (cost as f32) * 0.95;
        println!("Your total cost is {}", cost);
    }else{
        println!("Your total cost is {}", cost);
    }
}
