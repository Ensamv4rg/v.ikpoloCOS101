use std::io;
fn main() {
    let mut number = String::new();
    println!("Enter a Number: ");
    io::stdin().read_line(&mut number).expect("Invalid Input");
    let number:i32 = number.trim().parse().expect("Not a valid number");

    for x in 1..(number+1){
        println!("{} X {} = {}", x, number, x*number);
    }

}
