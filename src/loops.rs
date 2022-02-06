pub fn run() {
    // Infinite loop
    let mut count = 0;
    loop {
        count += 1;
        println!("Number {}", count);
        if count == 5 {
            break;
        }
    }

    // While loop just like javascript

    //For range loop
    for x in 0..100 {
        println!("For Range :{}", x);
    }
}
