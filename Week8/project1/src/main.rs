use std::io;
fn main() {
     //initialization of vectors//
    let mut names = Vec::new();
    let mut working_years:Vec<u8> = Vec::new();
    let mut office = Vec::new();
    let mut response = String::new();
    let offices:[String;4] = ["office administrator".to_string(), "academic".to_string(), "lawyer".to_string(), "teacher".to_string()];
    let mut role: Vec<String> = Vec::new();

    println!("How many employee's are being checked");
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let response:i32 = response.trim().parse().expect("Invalid Integer");//gets the number of employees being checked

    for x in 0..response{//for loop to gather information about employees name, number of years worked and the office,
        let index = x as usize;
        names.push(input(String::from(format!("What is employee {}'s name", x+1))));
        loop{
            office.push(input(String::from("What group does the employee fall under: Office Administrator, Academic, Lawyer, Teacher")));
            if offices.contains(&office[index].to_lowercase()){ //checks if the response is a valid response. 
            break;
        }else{
            println!("Invalid answer enetered. Offices available for now are Office Administrator, Academic, Lawyer and Teacher");
            office.pop();
        }//gets and checks if the information about the employees office is correct
        }
        let years = input(String::from("How many years has the employee worked?"));
        working_years.push(years.parse().expect("Invalid Integer"));
        staff_level(&office[index], working_years[index], &mut role);
    }

    for x in 0..response{
        println!("{} is serves in the {} department as a(n) {} because they have {} year(s) of experience", names[x as usize],office[x as usize],role[x as usize], working_years[x as usize]);//prints out all the information gathered about the employee. This gives the role the employee is supposed to serve in an office. This can be used to validate the employees role and office.
        println!("");
    }
}
fn input(question:String)-> String{
    let mut response = String::new();
    println!("{} ", question);
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let answer = String::from(response.trim());
    return answer;//function to ask questions and return the response
}

fn staff_level(office: &str, years_of_experience: u8, role: &mut Vec<String>) {
    match office.to_lowercase().as_str() {
        "office administrator" => admin(years_of_experience, role),
        "academic" => academic(years_of_experience, role),
        "lawyer" => lawyer(years_of_experience, role),
        "teacher" => teacher(years_of_experience, role),
        _ => (),
    }//determines the office of the employee and calls the appropraite function
}

fn admin(years_of_experience: u8, role: &mut Vec<String>) {
    role_match(years_of_experience, "Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO", role);
}

fn academic(years_of_experience: u8, role: &mut Vec<String>) {
    role_match(years_of_experience, "Not Applicable", "Research Assistant", "PhD Candidate", "Post Doc researcher", "Senior Lecturer", "Dean", role);
}

fn lawyer(years_of_experience: u8, role: &mut Vec<String>) {
    role_match(years_of_experience, "Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner", role);
}

fn teacher(years_of_experience: u8, role: &mut Vec<String>) {
    role_match(years_of_experience, "Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal", role);
}

fn role_match(years_of_experience: u8, case1: &str, case2: &str, case3: &str, case4: &str, case5: &str, case6: &str, role: &mut Vec<String>) {
    match years_of_experience {
        0|1|2 => role.push(case1.to_string()),
        3|4|5 => role.push(case2.to_string()),
        6|7 => role.push(case3.to_string()),
        8|9|10 => role.push(case4.to_string()),
        11|12|13 => role.push(case5.to_string()),
        _ => role.push(case6.to_string()),
    }//matches the years of experience with the appropriate role and pushes it to the role vector.
}

