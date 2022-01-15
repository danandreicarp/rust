use std::io;

pub fn read_int_array() -> Vec<i32> {
    println!("Please enter an array of values:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read values");

    let input_values: Vec<&str> = input.split_whitespace().collect();

    let mut array: Vec<i32> = Vec::new();

    for input_value in &input_values {
        let value: i32 = input_value
            .trim()
            .parse()
            .expect("Failed to convert value to int");
        array.push(value);
    }

    println!("Read values: {:?}", array);
    array
}

pub fn read_str_array() -> Vec<String> {
    println!("Please enter a sequence of words:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input_words: Vec<&str> = input.split_whitespace().collect();

    let mut array: Vec<String> = Vec::new();

    for word in input_words {
        let string = String::from(word);
        array.push(string);
    }

    array
}

pub fn read_number_input(prompt: &str) -> u8 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Failed to parse input")
}

pub fn read_string_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    String::from(input.trim())
}

pub fn read_string_tuple(prompt: &str, split_pattern: &str) -> (String, String) {
    println!("{}", prompt);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let tuple = input.split_once(split_pattern);

        match tuple {
            Some(pair) => return (String::from(pair.0.trim()), String::from(pair.1.trim())),
            None => {
                println!(
                    "Input failed to match the pattern. Please use '{}'",
                    split_pattern
                );
                continue;
            }
        }
    }
}
