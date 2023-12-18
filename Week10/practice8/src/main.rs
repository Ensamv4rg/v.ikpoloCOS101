Struct employee{
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    let emp1 = employee{
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };

    let emp2 = employee{
        company:String::from("Google Inc"),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    display(emp1);
    display(emp2);

}

fn display(emp:employee){
    println!("Name is: {} company is {} age is {}", emp.ceo, emp.company.emp.age);
}
