// Vectors are similar to arrays but they don't have fixed size
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4]; //This makes a vector but of i32 type
                                                  //Every this is same as array in javascript
    numbers.push(5); //adding at the last of the vector
    numbers.pop(); //deleting the last value

    // Loop through the values in the vector
    for num in numbers.iter() {
        println!("Each element of the vector: {}", num);
    }

    //Loop and mutate
    for num in numbers.iter_mut() {
        *num = *num * 2;
    }
    println!("Printing after mutating the vector {:?}", numbers);
}
