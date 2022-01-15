use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random(x: i32) -> i32 {
    let random: i32 = rand::random();
    println!("Adding {} + {}", x, random);
    x + random
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
