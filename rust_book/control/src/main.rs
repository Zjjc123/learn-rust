fn main() {
    let mut count = 5;
    loop {
        count -= 1;
        println!("count: {}", count);
        if count == 0 {
            break;
        }
    }

    while count < 4 {
        count += 1;
        println!("count: {}", count);
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // range (and reverse range)
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("Lift off!");
}
