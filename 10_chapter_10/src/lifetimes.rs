use std::fmt::Display;

pub fn lifetimes_1() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}

pub fn call_lifetimes() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    use_important_excerpt_1();

    let _s: &'static str = "I have a static lifetime.";

    longest_with_an_announcement(string1.as_str(), string2, "Food ready!");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn use_important_excerpt_1() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let level = i.level();
    println!("level: {}", level);

    let part = i.announce_and_return_part("Stay behind the yellow line");
    println!("part: {}", part);
}
