use crate::io::read_input;
mod io;
mod one;
mod three;
mod two;

fn main() {
    println!("");

    let option = read_input::read_number_input(
        "Program starting. Please select an option:\n
        1. Vector Operations\n
        2. Pig Latin\n
        3. Employees Database\n
        4. Quit",
    );

    println!("You chose option: {}", option);

    match option {
        1 => one::execute(),
        2 => two::execute(),
        3 => three::execute(),
        _ => return,
    }
}
