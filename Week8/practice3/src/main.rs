fn value(n:Option<&char>){
    println!("Element of vector {:?}", n);
}

fn main() {
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];

    let mut input1 = String::new();
    println!("\n Enter an index value btw (o to 8)");
    std::io.stdin().read_line(&mut input1).expect("Failed to read input");

    let index:usize = input1.trim().parse().expect("Invalid Input");

    let ch: Option<&char>  = v.get(index);
    value(ch);
}
