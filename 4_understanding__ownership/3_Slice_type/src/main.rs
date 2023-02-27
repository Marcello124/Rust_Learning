fn main() {
    let s = String::from("Hello, World!");
    let _word = first_word(&s);

    // s.clear(); // this throws error

    let _word = first_word(&s[0..6]);
    
    // these are the same
    let _word = first_word(&s[..]);
    let _word = first_word(&s);

    let string_literal = "hello, world!";

    let _word = first_word(&string_literal[0..6]);
    let _word = first_word(&string_literal[..]);

    // this works becouse string literal is already string slice
    let word = first_word(string_literal);

    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..2]; // this is also a slice

    assert_eq!(slice, [2]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
