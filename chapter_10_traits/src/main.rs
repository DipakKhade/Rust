use std::fmt::{Debug, Display};

pub trait Summary {
    fn summary(&self) {
        println!("This is a message from summary trait")
    }
}

struct Tweets {
    title: String,
    desc: String,
}

impl Summary for Tweets {}
fn main() {
    let tweet = Tweets {
        title: String::from("tweet1"),
        desc: String::from("asd"),
    };

    tweet.summary();
}

fn some_fucntion(t: &impl Summary) {}

fn another_funcation(t: &(impl Display + Clone), u: &(impl Display + Debug)) {}

fn another_funcation_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Display + Debug,
{
    32
}
