// Rust is a statically typed language
// it must know the types of all variables at compile time
// The compiler can usually infer what type we want to use 
// based on the value and how we use it. 

// In cases when many types are possible 
// we must add a type annotation

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // literals

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("decimal: {} hex: {} octal: {} binary: {} byte: {}", 
            decimal, hex, octal, binary, byte);
}
