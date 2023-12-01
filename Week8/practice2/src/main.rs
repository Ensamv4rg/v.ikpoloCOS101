use std::io;
fn main() {
    let v = vec!['C', 'O','M','P', 'U', 'T', 'E', 'R'];

    let mut input1 = String::new();

    println!("Enter an index value btw 0 and 7 ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read code");

    let index:usize = input1.trim().parse().expect("Invalid Input");
    let ch:char = v[index];

    print!("[] is the charachter for index [{}]\n", ch,index);
}
