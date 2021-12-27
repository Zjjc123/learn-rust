fn main() {
    // references allow us to refer to a value but not own it
    // & the ampersand syntax let us create a reference
    // the action of creating a reference is called "borrowing"
    // rerferences are immutable by default
    // can make references mutable with the mut keyword
    // &mut indicates a mutable reference
    // there can only be one mutable reference (eliminates data races)
    // cannot have mutable references while we have immutable ones
    let s1 = String:: from("hello");
    print_string(&s1);

    let mut s2 = String:: from("hello");
    change(&mut s2);
    print_string(&s2);
}

fn print_string(some_string: & String) {
    println!("some_string: {}", some_string);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
