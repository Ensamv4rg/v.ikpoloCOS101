use std::io;
fn checker(){
    let mut input = String::new();
    println!("Enter a charachter: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid Input");

    if ch >= '0' && ch <= '9'{
        println!("Charchter {} is a digit",ch);
    }else{
        println!("Chrachter {} is not a digit",ch);
    }
}
fn main(){
    println!("Welcome! This program checks whether a charachter variable contains a digit or not");
    checker();
}
