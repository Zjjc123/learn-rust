// Rust has a feature called automatic referencing and 
// dereferencing

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// associated functions
// associated function that don't have self 
// as their first parameter (not methods)
// often used for constructors that will return
// a new instance of the struct

// multiple impl block

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {}", rect1.area());
}