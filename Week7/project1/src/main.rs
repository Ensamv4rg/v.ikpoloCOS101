// Documentation: The program asks the user to make a selection out of 5 options to perform a respective geomteric calculation
//This code was made to be easily updated.
use std::io;//import for user input
use std::f64::consts::PI;//import for constant pi
fn main() {
    println!("What would you like to calculate. \n 1.Area of a Trapezoid. \n2.Area of a Rhombus \n3. Area of a Parallelogram \n4. Area of a cube \n5. Volume of a cylinder");
    println!();
    println!("Use the numbers to choose an option");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let mut response:i32 = response.trim().parse().expect("Invalid Integer");
    response = valid_response_check("Selection", response);
     calculator(response);
}

fn valid_response_check(response_type:&str, response:i32) ->i32 {// Checks to see if the input is one of the options
    let mut response = response;
    if response_type == "Selection"{
        loop{
            if response < 1 || response > 5{
                println!("The response you've entered is invalid. Pick a number from 1 to 5 to make a selection");
                let mut response_for_function = String::new();
                 io::stdin().read_line(&mut response_for_function).expect("Failed to read input");
                 response = response_for_function.trim().parse().expect("Not a valid integer");
             }else{
                break;
            }

        }

    }
    return response;
}
fn calculator(shape:i32){//calculates the area or volume of shape selected
// Strictly following constraints of the question, the maximum number of parameters a shape can have is 3. In future, if shapes with more than 3 parameters are added, it can be easily implemented without distrupting the functionality of the existing code.
    let mut parameter1 = String::new();
    let mut parameter2 = String::new();
    let mut parameter3 = String::new();//initialization of parameters
    let mut shape_parameters = [&mut parameter1, &mut parameter2, &mut parameter3]; //stores parameters in an array for easy acces for program and better organization

    let mut parameters_int = [0.,0.,0.]; //array storing the numeric values of the parameters
    let mut x = 0;//x is placeholder.
    ////////////////////////////////
    if shape == 1{ //Calculate area of a trapezoid
    let parameter_names = ["height","First base", "Second Base"];//sets the name of the parameters so the output can properly display what parameter the program asks for

        for x in 0..3{//
        println!("Enter {} of the trapezoid", parameter_names[x]); //parameternames[x] prints out the parameter(picked from parameter names) the computer seeks from the user
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input"); //reads the user input and stores it in it's respective space
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");//sets the parameters numeric value
    }
    println!( "The area of your trapezoid is {} units squared",parameters_int[0] * (parameters_int[1] + parameters_int[2]) * 0.5);//calculation


    }else if shape == 2{//calculate area of a Rhombus
        let parameter_names = ["First diagonal", "Second Diagonal"];
        for x in 0..2{
        println!("Enter {} of the Rhombus", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
    }
    println!( "The area of your rhombus is {} units squared",parameters_int[0] * parameters_int[1] * 0.5);

    }else if shape == 3{//Calculate Area of a Parallelogram

        let parameter_names = ["Base", "altitude"];
        for x in 0..2{
        println!("Enter {} of the parallelogram", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
    }
    println!( "The area of your paralleogram is {} units squared",parameters_int[1] * parameters_int[0]);

    }else if shape == 4{//Calculte Surface area of cube
        let parameter_names = ["Length"];
        println!("Enter {} of the cube", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
        println!( "The area of your cube is {} units squared",6. * (parameters_int[0] as f64).powf(2.));

    }else if shape ==5{//Calculate the volume of a cylinder
        let parameter_names = ["Radius", "height"];
        for x in 0..2{
        println!("Enter {} of the cylinder", parameter_names[x]);
        io::stdin().read_line(&mut shape_parameters[x]).expect("Invalid Input");
        parameters_int[x] = shape_parameters[x].trim().parse().expect("Invalid Integer");
    }
    println!( "The volume of yout cylinder  is {:.4}(4d.p) units cubed", PI * parameters_int[1] * (parameters_int[0] as f64).powf(2.));
     }
 }
