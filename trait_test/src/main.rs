extern crate trait_test;

use trait_test::test::NewsArticle;
use trait_test::test::Tweet;
use trait_test::test::notify;
use crate::trait_test::test::Summarizable;

fn main() {
    let news = NewsArticle {
        headline: String::from("a"),
        location: String::from("a"),
        author: String::from("a"),
        content: String::from("a")
    };
    println!("{}", news.summary());
    notify(news);
}
