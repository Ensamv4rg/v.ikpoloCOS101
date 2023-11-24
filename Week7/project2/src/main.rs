use std::io;
fn main() {
    let mut response = String::new();
    println!("How many siblings do you have? ");
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let number_of_siblings: u8 = response.trim().parse().expect("Invalid Integer");

    if number_of_siblings == 0 {
        println!("Your situation is not within the constraints of the program");
    } else if number_of_siblings > 50 {
        println!("Number of siblings is too large");
    } else {
        let number_of_siblings: usize = number_of_siblings.into();

        let mut names = vec![];
        let mut ages: Vec<u32> = vec![];
        let mut marital_status = vec!["n/a".to_string() ; number_of_siblings];
        let mut proffession = vec!["n/a".to_string(); number_of_siblings];
        let mut residence = vec!["n/a".to_string(); number_of_siblings];
        let mut university = vec!["n/a".to_string();number_of_siblings];
        let mut course_of_study = vec!["n/a".to_string(); number_of_siblings];
        let mut parental_status = vec!["n/a".to_string(); number_of_siblings];
        let mut waec_status = vec!["n/a".to_string(); number_of_siblings];
        let mut class = vec!["n/a".to_string();number_of_siblings];
        let mut secondary_school = vec!["n/a".to_string(); number_of_siblings];

        for placeholder in 0..number_of_siblings {

            let mut name_input = String::new();
            println!("What is the name of sibling {}", placeholder + 1);
            io::stdin().read_line(&mut name_input).expect("Failed to read input");
            let name = name_input.trim().to_string();
            names.push(name);

            let mut age_input = String::new();
            println!("How old is {} ", names[placeholder]);
            io::stdin().read_line(&mut age_input).expect("Failed to read input");
            let age: u32 = age_input.trim().parse().expect("Invalid Integer");
            ages.push(age);

            if ages[placeholder] > 18{
                cleaner_condition_handler_code(placeholder,format!("is {} married",names[placeholder]),"y","n",&mut marital_status,"Is Married", "Is Single");
                    if marital_status[placeholder] == "Is Single"{
                        cleaner_condition_handler_code(placeholder,format!("is {} a worker(w) or student(s)", names[placeholder]),"w", "s",&mut proffession, "Is a Worker", "Is a student");
                        if proffession[placeholder] == "Is a student"{
                            further_processing(placeholder, format!("What university is {} in", names[placeholder]), "Is a student".to_string(),&mut proffession,& mut university);
                            question(format!("What course is {} doing",names[placeholder]), placeholder,&mut course_of_study);
                        }
                        
                            
                    }else if marital_status[placeholder] == "Is Married"{
                        question(format!("What city does {} live in ", names[placeholder]),placeholder, &mut residence);
                        cleaner_condition_handler_code(placeholder, format!("is {} a parent", names[placeholder]),"y","n", &mut parental_status, "Has a child", "Doesn't have a child",);
                    }
            }else{
                cleaner_condition_handler_code(placeholder,format!("Has {} written WAEC", names[placeholder]),"y", "n",&mut waec_status, "Has written WAEC", "Has not written WAEC");
                if waec_status[placeholder] == "Has not written WAEC"{
                    question(format!("What class is {} in", names[placeholder]),placeholder, &mut class);
                }else if waec_status[placeholder] != "n/a"{
                    question(format!("What school did {} go to", names[placeholder]), placeholder, &mut secondary_school);
                }
            }
        }
        for element in 0..number_of_siblings{
            if ages[element] > 18{
                if marital_status[element] == "Is Married"{
                    if parental_status[element] == "Has a child"{
                        println!("{} is {} years old, is a parent, {} and lives in {}", names[element], ages[element], marital_status[element], residence[element]);
                    }else{
                        println!("{} is {}years old, is married but isn't a parent", names[element], ages[element]);
                    }
                }else{
                    if proffession[element] == "Is a Worker"{
                        println!("{} is {} years old and is a worker", names[element], ages[element]);
                    }else{
                        println!("{} is {} years old, is a student at {} and is studying {}", names[element], ages[element], university[element], course_of_study[element]);
                    }
                }
            }else{
                if waec_status[element] == "Has written WAEC"{
                    println!("{} is {} years old, has written WAEC and attended {} (High School)", names[element], ages[element], secondary_school[element]);
                }else{
                    println!("{} is {} years old and is currently in {}", names[element], ages[element], class[element]);
                }
            }
        }
    }
}
fn condition_handler(placeholder:usize,condition:String,option1:String,option2:String,
    vector:&mut Vec<String>,
    output1:String,output2:String){
    loop{
        let mut response = String::new();
        println!("{}? ({}/{}) ", condition, option1,option2);
        io::stdin().read_line(&mut response).expect("failed to read input");

        if response.to_lowercase().trim() != option1 && response.to_lowercase().trim() != option2{
            println!("Invalid Response");
        }else if response.to_lowercase().trim() == option1{
            vector[placeholder] = String::from(output1);
            break;
        }else if response.to_lowercase().trim() == option2{
            vector[placeholder] = String::from(output2);
            break;
        }
    }
}
fn cleaner_condition_handler_code(placeholder:usize, condition_text:String, first_option:&str, second_option:&str, vector:&mut Vec<String>, output1:&str, output2:&str){
    condition_handler(placeholder as usize , condition_text.to_string(), first_option.to_string(), second_option.to_string(),vector,output1.to_string(), output2.to_string());
}
fn further_processing(position:usize,question:String,argument:String, in_vector:&mut Vec<String>, out_vector:&mut Vec<String>){
    if in_vector[position] == argument{
        println!("{}", question);
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read response");
        let answer = response.trim().to_string();
        out_vector[position] = answer;

    }
}
fn question(question:String, position:usize, out_vector: &mut Vec<String>){
    println!("{}",question);
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let answer = response.trim().to_string();
    out_vector[position] = String::from(answer);
}





