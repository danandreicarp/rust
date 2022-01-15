use std::io;

fn main() {
    println!(
        "Please choose an option:\n 
    1. Celsius to Fahrenheit\n 
    2. Fahrenheit to Celsius\n
    3. Fibonacci\n
    4. Twelve Days of Christmas"
    );

    let option = read_input_as_int();

    println!("You entered {}", option);

    if option == 1 {
        celsius();
    } else if option == 2 {
        fahrenheit();
    } else if option == 3 {
        fibonacci();
    } else if option == 4 {
        twelve_days_of_christmas();
    } else {
        println!("Unknown Option: {}. Please enter 1, 2, 3 or 4.", option);
    };
}

fn celsius() {
    println!("Please enter the temperature in Celsius");

    let celsius = read_input();

    let celsius = celsius.trim().parse().expect("Failed to parse input");

    let fahrenheit = convert_to_fahrenheit(celsius);
    println!(
        "Fahrenheit value for Celsius of {} is {}",
        celsius, fahrenheit
    );
}

fn fahrenheit() {
    println!("Please enter the temperature in Fahrenheit");

    let fahrenheit = read_input();

    let fahrenheit = fahrenheit.trim().parse().expect("Failed to parse input");

    let celsius = convert_to_celsius(fahrenheit);
    println!(
        "Celsius value for Fahrenheit of {} is {}",
        fahrenheit, celsius
    );
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fibonacci() {
    println!("Please enter the number in the Fibonacci sequence you want returned (starting at 0)");

    let number = read_input_as_int();

    println!("You entered: {}", number);

    let result: u128;
    if number == 0 {
        result = 0;
    } else if number == 1 {
        result = 1;
    } else {
        let mut prev_2: u128 = 0;
        let mut prev_1: u128 = 1;
        let mut fibonacci: u128 = prev_2 + prev_1;
        for _index in 2..number {
            prev_2 = prev_1;
            prev_1 = fibonacci;
            fibonacci = prev_2 + prev_1;
        }
        result = fibonacci;
    }

    println!("The number you're looking for is: {}", result);
}

fn twelve_days_of_christmas() {
    println!("Lyrics for the \"Twelve Days of Christmas\" song:");

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "evelenth", "twelfth",
    ];

    for day in 0..=11 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[day]
        );

        for gift in (0..=day).rev() {
            println!("{}", gifts[gift].to_string());
        }
        println!();
    }
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn read_input_as_int() -> u8 {
    let input = read_input();
    input.trim().parse().expect("Failed to parse input")
}
