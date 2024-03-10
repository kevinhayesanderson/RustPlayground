mod string_collection;
mod vectors;
use std::collections::HashMap;

use string_collection as sc;
mod hash_maps;
use hash_maps as hm;

use ::function_name::named;

fn main() {
    println!("Hello, world!");
    vectors::create_new_vectors();
    vectors::updating_vectors();
    vectors::reading_vectors();
    vectors::iterating_values();

    sc::creating_strings();
    sc::updating_strings();
    sc::indexing_strings();
    sc::iterating_strings();

    hm::creating_hash_maps();
    hm::accessing_values();
    hm::iterating_hash_maps();
    hm::updating_hash_maps();

    let (median, mode) = exercise_1(&mut vec![
        1, 2, 3, 4, 5, 6, 7, 8, 2, 6, 6, 6, 4, 9, 9, 9, 9, 9, 9, 4,
    ]);
    println!("Median: {median}");
    println!("Mode: {mode}");

    let word = String::from("first");
    println!("Word: {word}");
    let pig_latin = exercise_2(word);
    println!("Pig Latin: {pig_latin}");
    let word = String::from("apple");
    println!("Word: {word}");
    let pig_latin = exercise_2(word);
    println!("Pig Latin: {pig_latin}");
}

/// exercise_1: median and mode
// Given a list of integers,
// use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
#[named]
fn exercise_1(values: &mut Vec<i32>) -> (i32, i32) {
    let function_name = function_name!();
    println!("Function: {function_name}");

    println!("Values before Sorted: {:?}", values);
    values.sort();
    println!("Values after Sorted: {:?}", values);

    let length = values.len();
    println!("length: {length}");
    let mid = length / 2;
    println!("mid: {mid}");
    let median = values[mid];
    println!("median: {median}");

    let mut hashmap = HashMap::new();
    for value in values {
        let count = hashmap.entry(value).or_insert(1);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    for (key, value) in &hashmap {
        if value > &max_count {
            max_count = *value;
            mode = **key;
        }
    }

    (median, mode)
}

/// exercise_2: convert strings to pig latin
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
#[named]
fn exercise_2(string: String) -> String {
    let function_name = function_name!();
    println!("Function: {function_name}");

    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for char in string.chars() {
        if vowels.contains(&char) {
            return format!("{string}-hay");
        } else {
            let first_consonant = String::from(char);
            let slice: String = string.chars().into_iter().skip(1).collect();
            return format!("{slice}-{first_consonant}ay");
        }
    }

    return String::new();
}
