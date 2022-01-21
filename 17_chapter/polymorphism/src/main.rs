use polymorphism::Draw;
use polymorphism::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

impl SelectBox {
    fn new(width: u32, height: u32, options: Vec<String>) -> Box<SelectBox> {
        Box::new(SelectBox {
            width,
            height,
            options,
        })
    }
}

pub fn main() {
    let select_box = SelectBox::new(10, 10, vec![]);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
