// function signature must declare the type of each parameter

fn main() {
    print_labeled_measurement(5, 'h');
    expression();
    println!("{}", return_five());
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// macros and functions are expressionsl

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("y: {}", y);
}

// return values (declare type)
fn return_five() -> i32 {
    5
}