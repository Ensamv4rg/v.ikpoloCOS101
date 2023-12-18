struct employee{
    name:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = employee{
        company:String::from("Ernst & Young"),
        name:String::from("Ebibong Jessica"),
        age:25
    };
    println!("Name = {} \n", emp1.name);
    println!("company = {} \n", emp1.company);
    println!("Age = {}", emp1.age);
}
