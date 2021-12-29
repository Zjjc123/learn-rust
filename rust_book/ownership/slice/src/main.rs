fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // create a slice by specifying [starting..ending]
    // stores a reference to the first element and a length
    // string literal are slices (of type &str)

    // array
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]);
    println!("first world: {}", first_word_slice(&s));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &itme) in bytes.iter().enumerate() {
        if itme == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &itme) in bytes.iter().enumerate() {
        if itme == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}