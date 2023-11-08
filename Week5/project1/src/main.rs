use std::io;
fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    println!("Enter the coeffecient of x^2: ");
    io::stdin().read_line(&mut a).expect("Not a valid input");
    let a:f32 = a.trim().parse().expect("Not a valid int");

    println!("Enter the coeffecient of x: ");
    io::stdin().read_line(&mut b).expect("Not a valid input");
    let b:f32 = b.trim().parse().expect("Not a valid int");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut c).expect("Not a valid input");
    let c:f32 = c.trim().parse().expect("Not a valid int");

    if (b.powf(2.) - (4.*a*c)) < 0.{
        let den:f32 = a * 2.;
        let mut real:f32 = -1. * b;

        let mut imaginary:f32 = -1. * (b.powf(2.) - (4.*a*c));
        imaginary = imaginary.sqrt();

        real = real/den;
        imaginary = imaginary/den;

         println!("The roots are {} + {}i and {} - {}i ", real, imaginary,real,imaginary);
 

    }else if (b.powf(2.) - (4.*a*c)) > 0.{
        let answer1 = (-b + ((b.powf(2.))- (4.*a*c)).sqrt())/(2.*a);
        let answer2 = (-b - ((b.powf(2.)) - (4.*a*c)).sqrt())/(2.*a);
        println!("The roors of the equation are {} and {}", answer1, answer2);
    }else{
        let answer = (-b + ((b.powf(2.))- (4.*a*c)).sqrt())/(2.*a);
        println!("x is :{}", answer);
    }



}
