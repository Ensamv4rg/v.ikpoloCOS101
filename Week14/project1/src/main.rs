use std::io;
use std::io::Write; 
use std::path::Path;
use std::io::Read;

fn main() {
 let designation = question_input(vec![format!("s"), format!("c"), format!("p"),format!("a"),format!("v")],"Are you a member of staff(S), Customer(C),Project Manager(P),Administrator(A) or Vendor(V)?");

 match designation.trim(){
    "s" => authentication("Staff_login_data.txt"),
    "c" => todo!(),
    "a" => authentication("Administrator_login_data.txt"),
    "v" => authentication("Vendor_login_data.txt"),
    "p" => authentication("ProjectManager_login_data.txt"),
    &_ => todo!(),

 };

}

fn question_input(options:Vec<String>,question:&str)->String{
    let mut response = String::new();
    let mut status = false;
    let mut valid_options_available = String::new();

    for option in &options{
        valid_options_available.push_str(format!(" {},",option).trim());
    };
    println!("{}", question);

    if !options.contains(&format!("Ignore")){
        while !status {
        
        io::stdin().read_line(&mut response).expect("Failed to read input");
        response = response.trim().to_string();
        status = options.contains(*&&response.to_lowercase());

        if !status{
            println!("You entered an invalid input. Use any of {} to select a valid option", valid_options_available);
            response.clear();
        }

        }
    }else{
        io::stdin().read_line(&mut response).expect("Failed to read input");
    };
    return response;
    
}


fn authentication(designation:&str){
    let path = Path::new(& *designation);
    if !path.exists(){
        let _file = std::fs::File::create(& *designation).expect("Failed to create file");
    };
    
    let username = question_input(vec![format!("Ignore")],"Enter Username");
    let password = question_input(vec![format!("Ignore")],"Enter Password");

    let mut file = std::fs::File::open(designation).expect("Failed to Read File");
    let mut login_data = String::new();
    file.read_to_string(&mut login_data).unwrap();

    if login_data.contains(&format!("Username:{}|Password:{}",username,password)){
        name_replacer(designation);
    }else if designation == "Vendor_login_data.txt"{
        register_vendor();

    }else{
        println!("Invalid Login..");
    };

}

fn database_returner(table:&str){
    let mut file = std::fs::File::open(& *table).expect("Database does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

}

fn name_replacer(name:&str){
    let mut output = "";
    match name{
        "Staff_login_data.txt" => output = "staff.sql",
        "Administrator_login_data.txt" => output = "database.sql",
        "Vendor_login_data.txt" => output = "dataplans.sql",
        "ProjectManager_login_data.txt" output = "project.sql",
         &_ => output = "",

    };
    database_returner(output);
}

fn register_vendor(){

    let mut file = std::fs::File::open("Vendor_login_data.txt").expect("File does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Invalid Login");

    let response = question_input(vec![format!("y"), format!("n")],"Would you like to register as a vendor?");
    let mut username = format!("");
    let mut password = format!("");

    if response == "y"{
        loop{
       

        username = question_input(vec![format!("Ignore")], "Enter Username");

        if !contents.contains(&username){
            break;
        }
        println!("Username {} already exists. Pick a new user",username);
    
    }
    password = question_input(vec![format!("Ignore")], "Enter Password");
    file.write_all(format!("Username:{}|Password:{}",username,password).as_bytes());

    println!("You're now required to login with your new username and password");
    authentication("Vendor_login_data.txt");
    }

    

    

}
