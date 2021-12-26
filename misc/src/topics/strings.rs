// primitive string: immutable fixed-length string somewhere in memory
// String type: growable, heap-allocated data structure - modify

pub fn run() {
    let hello = "Hello";
    let mut world = String::from("World!");
   
    // push char
    world.push('a');

    // push string 
    world.push_str("bcdef");

    // capacity (bytes)
    println!("capacity: {}", world.capacity());

    println!("{}{}", hello, world);
    println!("length: {}", hello.len());
}