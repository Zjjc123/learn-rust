// fixed lists of same data types

use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // single value
    println!("{}", numbers[0]);

    // array length
    println!("{}", numbers.len());
    
    // memory (stack allocated: bytes)
    println!("{}", mem::size_of_val(&numbers));

    // slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);
}