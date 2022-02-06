/*
type1:-Premitive str = Immutable fixed length string somewhere in memory
type2:-string = Growable , heap allowcated data structure - Use when you need to modify or own
String data
*/
pub fn run() {
    let type1 = "Hello";
    let mut type2 = String::from("World");
    type2.push('!'); //Push can only add one single char
    type2.push_str(" from Rust"); // push_str adds a string
    let len = type2.len(); //Length of type2
    let capacity = type2.capacity(); //Capacity in bytes
    let contains = type2.contains("world"); //If contains sub string
                                            // assert_eq!(2, type2.len()); // will give error as the type2.len() is not 2
    println!("{:?}", (type1, &type2, len, capacity, contains));
    for words in type2.split_whitespace() {
        // Don't know why but this for loop won't work after the print
        println!("{}", words);
    }
}
