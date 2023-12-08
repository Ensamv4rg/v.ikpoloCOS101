use std::io::Write;
use std::io;

fn main() {
    //initialization of vectors
    let mut names = vec![];
    let mut matric_numbers = vec![];
    let mut level = vec![];
    let mut department = vec![];
    let valid_levels = ["100".to_string(),"200".to_string(), "300".to_string(), "400".to_string(), "500".to_string()];

    let mut response = String::new();
    println!("How many people's info is being generated");
    io::stdin().read_line(&mut response).expect("Failed to read line");
    let response:usize = response.trim().parse().expect("Invalid Integer(Line 14)");

    //collecting information
    for numbers in 0..response{
        collect_info("name", &mut names);
        collect_info("matric_number", &mut matric_numbers);
        collect_info("department", &mut department);
        collect_info("level",&mut level);

        while !valid_levels.contains(&level[numbers]){//validation for level
            level.pop();
            println!("Sorry, you've entered an invalid level. The valid levels are {:?}", &valid_levels[..]);
            collect_info("level",&mut level);
        }
    }

    println!("Creating Spreadsheet....");
    spreadsheet_creator(names.clone(), matric_numbers.clone(), level.clone(), department.clone(), names.len());
    println!("Spreadsheet has been created.");
}


fn collect_info(info:&str, vector:&mut Vec<String>){
    let mut response = String::new();

    println!("what is your {}", info);
    io::stdin().read_line(&mut response).expect("Failed to read input");
    vector.push(response.trim().to_string());
}

fn spreadsheet_creator(names:Vec<String>, matric_numbers:Vec<String>, level:Vec<String>, department:Vec<String>, number_of_rows:usize){

    let vector_names = ["Name","Matric Number","Level","Department"];
    let vectors = [names,matric_numbers,level,department];
    let mut file = std::fs::File::create("Student_Information.xlm").expect("Failed to create file");
    file.write_all(b"\t PAU SIMS \n");

    for vector in vector_names{ //Adds the vectors as column heads. eg. Name    Matric Number   Level    Department
        file.write_all(format!("{}\t", vector).as_bytes()).expect("Failed to write to file");
    }
    file.write_all(format!("\n").as_bytes());

    for index in 0..number_of_rows{// 
        for vector in &vectors{// Inputs Data on a row based off a record in order Name, Matric Number, Level, Department.
            file.write_all(format!("{}\t", vector[index]).as_bytes());
        }
        file.write_all(format!("\n").as_bytes());
    }


}

