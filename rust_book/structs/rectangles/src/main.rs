#[derive(Debug)] // opt into debug functionality
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {}", area(width1, height1));

    // tuples
    let rect1 = (30, 50);
    println!("The area of the rectangle is {}", area_rect(rect1));

    // data type
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {}", area_obj(&rect2));
    println!("Rectangle: {:?}", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// no function overloading
fn area_rect(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_obj(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}