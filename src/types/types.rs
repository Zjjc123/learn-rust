// statistically typed language
// must know type at compile time
// but compiler can usually infer what type we want to use based 
// on the value and how we use it

/* Primitive Types

Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128 i128 (u : unsigned, i: regular int, number: # of bits)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Array
*/

pub fn run() {
    // implicity type (default i32)
    let x = 1;

    // explicity type
    let y: f32 = 2.5;

    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let have_friends: bool = true;

    // from expression
    let is_greater = 10 < 5;

    // character
    let character = 'a';
    let emoji = '\u{1FAC2}';

    println!("{:?}", (x, y, have_friends, is_greater, character, emoji));
}

