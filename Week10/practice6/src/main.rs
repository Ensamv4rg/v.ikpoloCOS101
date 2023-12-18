fn addone(e: &mut i32){
    *e+=1;
}

fn main() {
    let mut i = 3;
    addone(&mut i);
    println!("{}", i);
}
