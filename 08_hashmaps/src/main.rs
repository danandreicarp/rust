use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    // without insert-by-reference above, this doesn't compile: borrow of moved value
    println!("Field name: {}", field_name);
    // without insert-by-reference above, this doesn't compile: borrow of moved value
    println!("Field value: {}", field_value);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("No Blue team!"),
    }

    if let Some(score) = score {
        println!("Blue team score: {}", score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Red")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
