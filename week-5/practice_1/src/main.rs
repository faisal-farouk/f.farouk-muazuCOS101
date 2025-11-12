fn main() {
    let name = "Faisal Farouk";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni,addr);


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Scince and Technology";
    println!("Department: {}, \nSchool: {}", department,school);
}
