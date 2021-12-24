// variables hold primitive data or references to data
// immutable by default
// block-scoped language 

pub fn run() {
    // mutable and immutable variables
    let name = "Jason";
    let mut friends = 0; 

    println!("My name is {}", name);
    println!("and I have {} friend(s)!", friends);

    friends = 1;

    println!("and I have {} friend(s)!", friends);

    // constants
    const MAGIC_NUMBER: i32 = 001;
    println!("MAGIC_NUMBER is: {}", MAGIC_NUMBER);

    // assign multiple variables
    let ( friend1, friend2 ) = ("Hello", "Darkness");
    println!("{} {} my old friend", friend1, friend2);
}
