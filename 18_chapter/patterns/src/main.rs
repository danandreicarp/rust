fn main() {
    // if let
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favourite_color {
        println!("Using your favourite colour, {}, as the background", colour);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background colour");
        } else {
            println!("Using orange as the background colour");
        }
    } else {
        println!("Using blue as the background colour");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at the index {}", value, index);
    }

    // let
    let (x, y, z) = (1, 2, 3);
    println!("x = {}\ny = {}\nz = {}", x, y, z);

    let (x, y, ..) = (4, 5, 6);
    println!("x = {}\ny = {}\nz = {}", x, y, z);

    let point = (3, 5);
    print_coordinates(&point);

    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring structs
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // destructuring enums
    let msg = Message::ChangeColour(Colour::Hsv(0, 160, 255));
    // let msg = Message::Move { y: 10, x: 5 };

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        // Message::ChangeColour(r, g, b) => println!(
        //     "Change the colour to red {}, green {}, and blue {}",
        //     r, g, b
        // ),
        Message::ChangeColour(Colour::Rgb(r, g, b)) => {
            println!("Change the colour to red {}, green {} and blue {}", r, g, b)
        }
        Message::ChangeColour(Colour::Hsv(h, s, v)) => println!(
            "Change the colour to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }

    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}", feet, inches);

    // ignoring an entire value with _
    foo(3, 4);

    // ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is: {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }

    // ignoring an unused variable by starting its name with _
    let _x = 5;
    let y = 10;

    // ignoring remaining parts of a value with ..
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { y, .. } => println!("y is {}", y),
    }

    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }

    // match guards
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ bindings
    let msg = ShortMessage::Hello { id: 5 };
    match msg {
        ShortMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        ShortMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        ShortMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

// function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

#[allow(unused)]
enum Colour {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(Colour),
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

#[allow(unused)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum ShortMessage {
    Hello { id: i32 },
}
