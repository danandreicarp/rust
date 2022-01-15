pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    }
}

// short syntax; can't restrict both items to the same type.
// pub fn notify(item1: &impl Summary, item2: &impl Summary);

// trait bound syntax; both items have the same type
// pub fn notify<T: Summary>(item1: &T, item2: &T);

// multiple trait bounds
// pub fn notify(item: &(impl Summary + Display));
// pub fn notify<T: Summary + Display>(item: &T);

// 'where' clause for trait bounds
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32;
// vs
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
