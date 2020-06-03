use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline, self.author, self.location
        )
    }

    fn summarize_author(&self) -> String {
        unimplemented!()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /// fn summarize from default trait impl!

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: "lorem".to_string(),
        location: "NY".to_string(),
        author: "John Doe".to_string(),
        content: "Lorem ipsum dolor".to_string()
    };

    notify(article);
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// alter
// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }


/// where syntax
fn some_fn<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    /// implementation
    return 42;
}