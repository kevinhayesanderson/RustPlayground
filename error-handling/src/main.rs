use std::io::ErrorKind;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {

    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn panic_in_our_code() {
    panic!("crash and burn");
}

fn panic_in_external_code() {
    let v = vec![1,2,3];
    v[99];
}

fn recoverable_errors_with_result() {
    // Recoverable Errors with Result
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error);
        }
    });
}


use std::io::{self, Read};
// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt");
    let mut username = String::new();
    username_file.unwrap().read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

use std::fs;

fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
