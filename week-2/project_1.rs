fn main() {
    
    let p: f64 = 520_000_000.0;  // Principal ₦520,000,000
    let r: f64 = 10.0;           // Rate (10%)
    let n: f64 = 5.0;            // Time in years (5 years)

    // Using the formula given
    let a = p * (1.0 + r / 100.0).powf(n);

    let ci = a - p;

    println!("Principal (P): ₦{:.2}", p);
    println!("Amount after {} years (A): ₦{:.2}", n, a);
    println!("Compound Interest (CI): ₦{:.2}", ci);
}
