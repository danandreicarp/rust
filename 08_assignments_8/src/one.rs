use crate::io::read_input;
use std::collections::HashMap;

pub fn execute() {
    let array = read_input::read_int_array();

    let mean = calculate_mean(&array);
    let median = calculate_median(&array);
    let mode = calculate_mode(&array);

    println!("mean: {}\nmedian: {}\nmode: {}", mean, median, mode);
}

fn calculate_mean(array: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for value in array {
        sum += value;
    }
    let sum: f64 = sum.into();
    let length: i32 = array.len().try_into().unwrap();
    let length: f64 = length.into();

    sum / length
}

fn calculate_median(array: &Vec<i32>) -> i32 {
    let mut sorted_array: Vec<i32> = array.to_vec();
    sorted_array.sort_unstable();

    sorted_array[sorted_array.len() / 2]
}

fn calculate_mode(array: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for value in array {
        let count = counts.entry(value).or_insert(0);
        *count += 1;
    }

    let mut max: (i32, i32) = (0, 0);
    for (key, value) in counts {
        if value > max.1 {
            println!("max was {:?}", max);
            max = (*key, value);
            println!("max becomes {:?}", max);
        }
    }

    max.0
}
