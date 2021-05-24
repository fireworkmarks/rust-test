use std::fs::read;

fn main() {
    let mut s = String::from("Hello world");
    let word_index = first_world(&s);

    s.clear();
    println!("{}", word_index);

    let s = String::from("Hello world");

    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];
    println!("{}, {}", hello, world);
    println!("{}", whole);

    let mut s = String::from("Hello world");
    let word_index_new = first_world_new(&s);

    /*
        here not use s.clear().
        cannot borrow 's' as mutable because it is also borrowed as immutable.
     */
    println!("{}", word_index_new);

    let my_string_literal = "Hello world";
    let word_index = first_world_new(my_string_literal);
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_world_new(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}