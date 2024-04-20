mod aggregator;
pub use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    fn returns_summarizable() -> impl Summary {//return type is trait
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    returns_summarizable().summarize();

    fn notify(item: &impl Summary){ //impl Trait syntax
        println!("Breaking news! {}", item.summarize());
    }

    fn notify1<T: Summary>(item: &T) //Trait Bound Syntax
    {
        println!("Breaking news! {}", item.summarize());
    }

    fn notify2(item: &(impl Summary + Display)) {}
    fn notify3<T: Summary + Display>(item: &T) {}

    notify(&article);
    notify1(&article);
    notify2(&tweet);
    notify3(&tweet);
}

pub trait Debug { }
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
{
    5
}
fn some_function_1<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    5
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
// and are extensively used in the Rust standard library.
// impl<T: Summary> ToString for T {
//     fn to_string(&self) -> String {
//         String::from("")
//     }
// }