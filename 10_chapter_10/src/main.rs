use chapter_10;
use chapter_10::summary::{NewsArticle, Summary, Tweet};

mod largest;
mod lifetimes;

fn main() {
    use_summary();

    largest::call_largest();
    lifetimes::call_lifetimes();
}

fn use_summary() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available: {}", article.summarize());
}
