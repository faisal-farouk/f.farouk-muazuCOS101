use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Amount of hours worked: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let hours:i32 = input2.trim().parse().expect("Not a valid number");

    if hours <= 40 {
        let g:i32 = hours * 3000;
        println!("Total gross salary: ${}", g);
    } else if hours > 40 {
        let g:i32 = hours * 4500;
        println!("Total gross salary: ${}", g);
    if g > 100_000 {
        let n:i32 = g - 2000;
        println!("Total net pay: ${}", n);
    } else {
        let n:i32 = g;
        println!("Total net pay ${}", n);
    }
}

}
