use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!(
        "The longest string is {} with length {}.",
        result,
        result.len()
    );

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("The first sentence is \"{}\"", i.part);
}

//The longest function definition specifying that all the references in the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Rust can’t tell whether the reference being returned refers to x or y.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
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

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
