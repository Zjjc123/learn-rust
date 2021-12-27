fn main() {
    let x = 0;
    // cannot reassign default immutable vairables 
    // x = 1;
    println!("x: {}", x);

    // shadowing variables
    let spaces = "       ";
    let spaces = spaces.len();
    println!("there are {} spaces", spaces);

    // can change type with shadowing but cannot mutate type
}
