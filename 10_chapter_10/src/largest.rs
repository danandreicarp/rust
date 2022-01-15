pub fn call_largest() {
    let numbers_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("The largest number is {}", largest(&numbers_list));
    println!("The largest char is {}", largest(&char_list));

    println!("The largest_ref number is {}", largest_ref(&numbers_list));
    println!("The largest_ref char is {}", largest_ref(&char_list));

    println!(
        "The largest_clone number is {}",
        largest_clone(&numbers_list)
    );
    println!("The largest_clone char is {}", largest_clone(&char_list));
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}
