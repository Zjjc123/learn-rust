pub fn run() {
    // print to console
    println!("Hello from print.rs");
    
    // basic formatting
    println!("Printing the number {} and {}", 1, 2);

    // positional argument
    println!("{0} + {0} = {1}", 1, 2);

    // named arguments
    println!("{name} goes to {school}", name = "Jason", school = "University of Washington");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x}", 69, 420);

    // placeholder for debug trait (tuple)
    println!("{:?}", (12, true, "hello"));
}
