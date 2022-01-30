use std::mem;
pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //This how declare an array. It should have fixed sized and datatype
    println!("Whole array{:?}", numbers); //Prints the whole array

    let mut numbers2: [i32; 5] = [1, 2, 3, 4, 5]; //This is a mutable array; size is not changable but values are
    numbers2[1] = 20; //Reassigning a single value
    println!("One element of the array {}", numbers2[1]); //Printing a single value
    println!("This array takes {} bytes", mem::size_of_val(&numbers2)); // Std is imported in line no.1

    let slice: &[i32] = &numbers2[0..2]; //This will slice the numbers2 from 0-1 total 2 elements and store it in slice
    println!("Slice of array {:?}", slice);
}
