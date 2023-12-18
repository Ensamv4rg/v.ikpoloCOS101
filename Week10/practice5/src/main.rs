fn main() {
    let x = vec![100,200,300];
    borrowvector(&x);

    println!("Printing the value from main() x[0] = {} ", x[0]);
    println!("******************************");
}

fn borrowvector(z:&Vec<i32>){
    println!("*******************************");
    println!("Inside Print vector function {:?} \n", z);
    println!("++++++++++++++++++++++++++++++++");
}
