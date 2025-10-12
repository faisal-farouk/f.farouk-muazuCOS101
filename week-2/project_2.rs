fn main() {
    // sales amounts (₦)
    let amounts = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    let sum: f64 = amounts.iter().sum();
    let average = sum / (amounts.len() as f64);

    println!("Total Sales: ₦{:.2}", sum);
    println!("Average Sales: ₦{:.2}", average);
}
