use add_one::{self, add_random};
use rand;

fn main() {
    let num = rand::random();
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );

    println!(
        "Hello, world! {} plus two is {}!",
        num,
        add_two::add_two(num)
    );

    println!("Hello, world! {} plus random is {}!", num, add_random(num));
}
