fn main() {
    let v = vec![15,25,35,45,55];
    printvector(v);
    println!("{}", v[0]);


}

fn printvector(x:Vec<i32>){
    println!("Inside print vector function {:?}", x);
}
