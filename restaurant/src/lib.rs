mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        //back_of_house::Breakfast has a private field,
        //the struct needs to provide a public associated function that constructs an instance of Breakfast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//public API, so we mark it with the pub
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    //The front_of_house module is defined within the same module as eat_at_restaurant,
    //so the relative path starting from the module in which eat_at_restaurant is defined works.

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //println!("Ooh I got {} fruit!", meal.seasonal_fruit);//can't access or modify it

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//While front_of_house isn’t public,
//because the eat_at_restaurant function is defined in the same module as front_of_house
//(that is, eat_at_restaurant and front_of_house are siblings),
//we can refer to front_of_house from eat_at_restaurant.

use crate::front_of_house::hosting; //hosting is now a valid name in that scope,

pub fn eat_at_restaurant_1() {
    hosting::add_to_waitlist();
}

mod customer {
    //should import hosting to use inside customer module
    use crate::front_of_house::hosting;

    //idiomatic way to bring a function into scope with use
    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        //use within the customer module
        hosting::add_to_waitlist();

        // reference the shortcut in the parent module with super::hosting within the child customer module.
        super::hosting::add_to_waitlist();

        add_to_waitlist(); //is unclear as to where add_to_waitlist is defined.
    }
}

//when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.
use std::collections::HashMap; //Bringing HashMap into scope in an idiomatic way

//There’s no strong reason behind this idiom:
//it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

//The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult;
//IoResult for the std::io::Result type, which won’t conflict with the Result from std::fmt that we’ve also brought into scope

fn function3() -> Result {
    // --snip--
    Ok(())
}

fn function4() -> IoResult<()> {
    // --snip--
    Ok(())
}

//////////////////////////////////////////////////////////////////////////
// Re-exporting Names with pub use
//  pub use crate::front_of_house::hosting;

// Before this change,
// external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// Now that this pub use has re-exported the hosting module from the root module,
// external code can now use the path restaurant::hosting::add_to_waitlist() instead.
//////////////////////////////////////////////////////////////////////////
// Using Nested Paths to Clean Up Large use Lists
//1.
// use std::cmp::Ordering;
// use std::iter;

use std::{cmp::Ordering, iter}; //Specifying a nested path to bring multiple items with the same prefix into scope

//2.
// use std::arch;
// use std::arch::x86_64;

use std::arch::{self, x86_64}; //This line brings std::arch and std::arch::x86_64 into scope.

//3.
//The Glob Operator
use std::collections::*; //to bring all public items defined in a path into scope
                         // Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
fn dummy() {}
//////////////////////////////////////////////////////////////////////////
