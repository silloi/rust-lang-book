fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut index1 = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if index1 == 0 {
                index1 = i;
            } else {
                return &s[index1..i];
            }
        }
    }

    if index1 == 0 {
        &s[..]
    } else {
        &s[index1..]
    }
}

fn main() {
    // let mut s = String::from("hello world");
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

    let word = first_word(&s);
    let second_word = second_word(&s);

    // s.clear();

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}
