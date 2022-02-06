pub fn run() {
    greeting("Good Morning", "Sourav");
    // binding func to a variable
    let sum = add(5, 5);
    println!("variable sum {}", sum);
    println!("Direct func sum {}", add(5, 5));

    // Closure in rust=> suppose I want to use the n1 and n2 variable of the add function out side
    // Did not get much about closure in rust.
    let n3 = 10;
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum {}", add_sum(5, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

// this -> means I will return a value which will be i32
// now I haven't put any semicolon after n1+n2 that means function will return that value
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
