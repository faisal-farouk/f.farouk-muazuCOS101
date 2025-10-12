fn main() {

	let p: f64 = 510_000.0;
	let r: f64 = 5.0;
	let n: f64 = 3.0;

	let a = p * (1.0 - r / 100.0).powf(n);
	let depreciation = p - a;

	println!("Original Value (P): ₦{:.2}", p);
	println!("Value after {} years (A): ₦{:.2}", n, a);
	println!("Total Depreciation: ₦{:.2}", depreciation);


}