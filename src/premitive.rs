/*
Premitive Types:-
Integer:- u8,i8,u16,i16,u32,i32,u64,i64,u128,i128- u means the integers can't have negative values
Floats:- f32,f64
Boolean:- (bool)
Characters:- (char)**Not a string but a single character**
Tuples
Array **Fixed length**
*/

// Rust is a statically typed language , which means it must know the type of the variables at compile time
// But the compailer can infer what type we want to use based on the value we use and how we use it.
// We don't need to explecitly mention it in all variables

pub fn run() {
    //X has a default type of i32
    let x = 2;
    // Y has a default type of f64
    let y = 2.5;

    //Adding explecit type
    let z: i64 = 20;

    println!("Max vaule of i32 {}", std::i32::MAX);
    println!("Max vaule of i64 {}", std::i64::MAX);

    let is_active: bool = false;
    let character: char = 'a';
    let face: char = '\u{1F600}';

    println!("Print all {:?}", (is_active, x, y, z, character, face));
}
