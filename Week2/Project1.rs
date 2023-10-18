fn main(){

	let l = 520000000.0; //L for loan
	let t = 5.0; //t for time in (years)
	let r = 10.0; //t for rate in %

	let a:f64 = 1. + (r/100.);// here, I split the formula into 2 segments to enable me call the powf function as rust doesn't use ^ for exponential power.

	

	let b = a.powf(t);
	

	let ci = b*l;
	let ci = ci - l; 


	println!("The compound interest is ${:.2}", ci);// by using {:.2}, I make sure my final answer is displayed in 2dp which is the appropriate numbers of decimals for money.

}