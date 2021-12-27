fn main() {
    // ===== move =====
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is no longer valid after calling "let s2 = s1;" 
    // eliminated double free errors

    // println!("{}, world!", s1); this is not a valid statement
    println!("{}, world!", s2); // but this is a valid statement

    // there is no "shallow copy" but it is called "move"
    // when s2 goes out of scope, the memory will be freed

    // stack copy is allowed
    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y);

    // ===== deep copy =====
    let s1 = String:: from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // ==== function =====
    // assing a variable to a function will move (pointer) or copy (value)

    let s = String::from("hello"); 
    takes_ownership(s);
    
    // println!("s: {}", s); is an invalid statement because
    // s no longer has ownership of the memory it is
    // moved into the function

    let x = 5;
    makes_copy(x);        

    // returning ownership
    // returning value transfers ownership
    
    let s1 = gives_ownership();
    let s2 = String::from("hello"); 
    let s3 = takes_and_gives_back(s2);

    println!("s1: {} s3: {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} 

fn gives_ownership() -> String { 
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}