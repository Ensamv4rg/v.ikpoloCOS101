use std::io;
fn main() {
    let mut number_of_candidates = String::new();
    println!("How many candidates have registered");
    io::stdin().read_line(&mut number_of_candidates).expect("Invalid Input");
    let number_of_candidates:i32 = number_of_candidates.trim().parse().expect("Invalid Integer");

    if number_of_candidates > 150{
        println!("Sorry, you aren't eligible to vote; The maximum number of candidates has been reached");
    }else{
        let mut level = String::new();
        loop{//this loop statement serves the purpose for error handling. It checks if the level the user has inputed is one of the 5valid levels at PAU. If it isn't, it rejects it and asks the user to re enter a valid level
            let mut level_forloop = String::new();// I initialized this variable as initializing the variable "level" within this loop statement, will not allow me to use it outside the loop. This error could probably also be combatted by creating a seperate function and having it return values. However, to limit the program to just one function, I feel this is the best possible apporach.
            println!("What level is the candidate at");
            io::stdin().read_line(&mut level_forloop).expect("Invalid Input");
            level_forloop = level_forloop.trim().to_string();

            if level_forloop != "100"&& level_forloop!= "200" && level_forloop != "300" && level_forloop != "400" && level_forloop != "500"{
                println!("Enter a valid level eg, 100, 200, 300, 400 or 500");
                println!("You typed the level is ({})", level_forloop);
            }else{
                level = level_forloop.to_string();
                break;
            }
        }
        if level == "100"{
            println!("Sorry, You aren't eligible. Try again next year, when you aren't in 100 level");
        }else{
            let mut cgpa = 0.0;
            loop{//this checks if the cgpa entered is valid. If it's less than 0.0, it asks the user to recheck, if it's greater than 5.0 also asks the user to recheck
                let mut cgpa_forloop = String::new();
                println!("Please enter your cgpa");
                io::stdin().read_line(&mut cgpa_forloop).expect("Invalid Input");
                let cgpa_forloop:f32 = cgpa_forloop.trim().parse().expect("Not a valid float");
                if cgpa_forloop < 0.0 || cgpa_forloop > 5.0{
                    if cgpa_forloop < 0.0{//this is used to tell the user that the cgpa entered is either too low or too high
                        println!("You aren't that dumb; your cgpa is literally below 0! check again");
                    }else{
                        println!("You might be smart, but it's impossible your cgpa is above 5.0, please check");
                    }
                }else{
                    cgpa = cgpa_forloop;
                    break;
                }
            }
            
            if cgpa < 4.0{
                println!("Sorry, you aren't eligible. Your cgpa is too low, try harder to improve it");
            }else{
                loop{//this loop serves the same purpose as the others. Except instead of breaking as soon as the value it checks, it rather allows the program to run if the input is valid
                    let mut response = String::new();
                    println!("Are you a class rep, (y/n)");
                    io::stdin().read_line(&mut response).expect("Invalid Integer");

                    if response.to_lowercase().trim() != "y" && response.to_lowercase().trim() != "n"{
                        println!("You've entered an invalid response. Type y for yes, n for no");
                    }else if response.to_lowercase().trim() == "y"{
                        println!("You can vote. But we need to collect some more information first");

                        let mut name = String::new();
                        println!("What is your name");
                        io::stdin().read_line(&mut name).expect("Invalid input");

                        let mut email = String::new();

                        loop{//error handling to check if the email is valid by checking if it has a .com or an @....it is slightly flawed for 2 reasons. Not all emails end in .com and the second being is it doesn't check the position of the @ meaning an invalid email can still be typed. A best option might be to create a function that attempts to send an email to the user. and if it's an error, it returns invalid email and prompts the user to retype. However, I am currently limited by the "technology of my time"
                            let mut email_forloop = String::new();
                            println!("What is your email");
                            io::stdin().read_line(&mut email_forloop).expect("Invalid input");

                            if email_forloop.trim().contains("@") && email_forloop.trim().ends_with(".com"){
                                email = email_forloop;
                                break;
                            }else{
                                println!("You've entered an invalid email. Please note this email only supports emails that end in .com");
                            }

                        }

                        

                        let mut department = String::new();
                        println!("What is your department");
                        io::stdin().read_line(&mut department).expect("Invalid Input");

                        let mut state_of_origin = String::new();
                        println!("What is your state of origin");
                        io::stdin().read_line(&mut state_of_origin).expect("Invalid Input");

                        println!("_________________Candidate Info______________ \n Name:{} \n Email:{} \n Department: {} \n State of Origin: {} \n This candidate is eligible to vote \n _____________________________________________ ", name, email, department,state_of_origin);


                        


                        break;
                    }else{
                        println!("Sorry, you aren't eligible. You need to be a class rep to run");
                        break;
                    }



                }

                
            }
        }
    

    


    


     }
}
