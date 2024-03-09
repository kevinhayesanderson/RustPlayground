use ::function_name::named;

pub fn creating_strings() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    println!("{s}");
}

pub fn updating_strings() {
    //append string
    let mut s = String::from("foo");
    s.push_str("bar");

    //append character
    let mut s = String::from("lo");
    s.push('l');

    //Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       //s2 has an &, meaning that we’re adding a reference of the second string to the first string.
                       //we can only add a &str to a String;

    let t: String = String::from("test string");
    let s4 = s3 + " " + "test string";
    println!("{s4}");
    let s5: String = s4 + " " + &t;
    println!("{s5}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

pub fn indexing_strings() {
    let hello = "Здравствуйте";
    //let answer = &hello[0]; //Rust strings don’t support indexing.

    //characters + diacritics = grapheme clusters

    //Slicing Strings
    let answer = &hello[0..2]; //two bytes for first character
                               //not always two bytes, that's why indexing is no allowed
    println!("{answer}");
}

#[named]
pub fn iterating_strings() {
    let function_name = function_name!();
    println!("{function_name}");

    let hello = "Здравствуйте";

    for c in hello.chars() {
        println!("{c}")
    }

    let hello = String::from("नमस्ते");
    for c in hello.chars() {
        println!("{c}") // can see the diacritics part from the characters
    }

    for b in hello.bytes() {
        println!("{b}")
    }
}
