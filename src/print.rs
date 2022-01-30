pub fn run() {
    println!("Hello World from print.rs file"); // This will print in the main.rs file only pub files can be imported
    println!("{}", 1); // For printing numbers have to use "{}"
    println!("{}  is my roll number {}", 1, 2); // The first {} will take 1 and the second will take 2
    println!(
        // This is called positional arguments
        "Hello I am {0} {1}. The meaning of {0} is Fregreance",
        "Sourav", "Das"
    );
    //This is called Named arguments
    println!(
        "My name is {name} {surname}",
        name = "Sourav",
        surname = "Das"
    );

    //Placeholder Traits
    println!("Binary: {:b}, Hexa: {:x}, Octal: {:o}", 100, 100, 100);

    // Placeholder for debug Traits
    println!("{:?}", (12, true, "ABCD")); // All different values come under :?

    //Basic maths
    println!("10+10={}", 10 + 10);
}
