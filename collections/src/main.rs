mod string_collection;
mod vectors;
use string_collection as sc;

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
}
