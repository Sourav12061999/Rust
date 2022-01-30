//Variables hold premitive data or refrences to data
//Variables are mutable by default
//Rust is a block scope language
pub fn run() {
    let name = "Sourav Das"; // This variable is immutable
    let mut age = 22; // With this mut keyword it is now mutable
    println!("My name is {} and I am {}", name, age);
    age = 23;
    println!("My name is {} and I am {}", name, age);

    //Const must have a type
    const DOB: &str = "12-06-1999"; //This variable name have to be in Uppercase
    println!("My Date of Birth is : {}", DOB);

    //Assigning multiple variables
    let (my_name, my_age) = ("Sourav Das", 23);
    println!("My name is : {} and I am {}", my_name, my_age);
}
