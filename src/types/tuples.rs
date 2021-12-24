// groups of different values
// max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Jason", "WA", 99);

    println!("{} is from {} and is level {}", person.0, person.1, person.2);
}