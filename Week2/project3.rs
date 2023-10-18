fn main(){
	let p = 210000.;
	let d = 5.;
	let t = 3.;

	let a:f64 = 1. - (d/100.);

	let b = a.powf(t);
	let c = p*b;

	let d = p - c;
	println!("The depreciation of the Tv set is: {:.2}",d);


}