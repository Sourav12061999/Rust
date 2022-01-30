//Tuples group togather values of different typs
// Max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("Sourav", "Das", 22); //This is a tuple
    println!(
        "My name is {} {} and my age is {}",
        person.0, person.1, person.2
    );
}
