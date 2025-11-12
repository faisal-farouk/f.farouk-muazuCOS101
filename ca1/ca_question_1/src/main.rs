use std::io;

fn main() {

    let mut input1 = String::new();

    println!("Enter Temperature in Celcius: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut c:f64 = input1.trim().parse().expect("Not a valid number");

    let f:f64 = (9.0 / 5.0) * c + 32.0;
    let k:f64 = c + 237.15;
    
    if c >= -273.0
    {
    println!("Temperature in Fahrenheit: {}f degrees", f);
    println!("Temperature in Kelvin: {}K", k);
    } else {
        println!("Invalid Temperature");
    }

    if c < 0.0 && c >= -273.0
    {
        println!("Freezing Point");
    }
    else if c >= 0.0 && c <= 30.0
    {
        println!("Normal Range");
    }
    else if c > 30.0
    {
        println!("Hot Temperature");
    }
    
}
