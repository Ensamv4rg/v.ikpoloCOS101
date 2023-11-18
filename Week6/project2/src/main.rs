use std::io;
fn main() {
    let mut numberOfResearchers = String::new();
    println!("How many researches have been inputed");
    io::stdin().read_line(&mut numberOfResearchers).expect("Inavlid Input");
    let numberOfResearchers:i32 = numberOfResearchers.trim().parse().expect("Not a valid number");


    if numberOfResearchers > 500{
        println!("Sorry, the research queue is full");
    }else{
        println!("Enter the reasearcher's name");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Invalid Input");


        let mut numberOfPapers = String::new();
        println!("How many papers has {} produced", name.trim());
        io::stdin().read_line(&mut numberOfPapers).expect("invalid input");
        let numberOfPapers:i32 = numberOfPapers.trim().parse().expect("invalid input");

        if numberOfPapers>= 10{
            println!("{} has an incentive of 1000000.00", name.trim());
        }else if numberOfPapers < 3{
            println!("{} has an incentive of 100000", name.trim());
        }else if numberOfPapers > 5{
            println!("{} has an incentive of 800000", name.trim());
        }else if numberOfPapers <= 5{
            println!("{} has an incentive of 500000", name.trim());
        }
    }



}
