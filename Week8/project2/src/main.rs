use std::io;
fn main() {

    let mut names:Vec<String> = vec![];
    let mut ages:Vec<u8> = Vec::new();
    let mut instances = Vec::new();
    let loop_number:usize = input(String::from("How many employees are being checked")).trim().parse().expect("Invalid Integer");
    


    for index in 0..loop_number{

        let x = index as i32;
        names.push(input(String::from(format!("What is the name of employee {}", (x + 1)))));
        ages.push(input(String::from(format!("How many years of experience does {} have", names[index].trim()))).trim().parse().expect("Invalid Integer"));

    }
    let largest_number_index = largest_number_finder(ages.clone());
    for x in 0..names.len(){
        if ages[x] == ages[largest_number_index]{
            instances.push(names[x].trim());
        }
    }

    if instances.len() == 1{
        println!("The employee with the most experience is {}", instances[0]);
    }else{

        println!("The employees with the most experience are:");
        for x in instances{
        println!("{}",x);
        }
    }

    
}

fn input(question:String)->String{

    println!("{}", question);
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read response");
    return response;
}

fn largest_number_finder(ages:Vec<u8>)->usize{
    let mut largest_number_index = 0;
    let mut check = 1;

    for _ in &ages{
        if check == ages.len(){
            break;
        }else if ages[largest_number_index] < ages[check]{
            largest_number_index = check;
            check = largest_number_index + 1;
        }else{
            check +=1;
        }

}
return largest_number_index;
}




