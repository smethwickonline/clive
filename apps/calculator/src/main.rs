use std::fmt::Display;

use druid::{
    widget::{Button, Flex},
    AppLauncher, Data, Size, Widget, WindowDesc,
};

#[derive(Data, Clone)]
enum Operation {
    Plus(f64),
    Minus(f64),
    Divide(f64),
    Multiply(f64),
}

#[derive(Data, Clone)]
struct State {
    value: f64,
    display: String,
    operation: Operation,
}

impl State {
    fn do_thing(&mut self) -> &mut Self {
        self.value = match self.operation {
            Operation::Plus(value2) => self.value + value2,
            Operation::Minus(value2) => self.value - value2,
            Operation::Divide(value2) => self.value / value2,
            Operation::Multiply(value2) => self.value * value2,
        };
        self.display = self.value.to_string();
        self
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

fn main() {
    let mut state = State {
        value: 12.0,
        display: "0".into(),
        operation: Operation::Divide(2.0),
    };

    let main_window = WindowDesc::new(ui_builder())
        .title("Calculator")
        .window_size(Size::new(640.0, 360.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("darn");
}

// fn number_button() -> impl Widget<State> {
//     todo!()
// }

fn ui_builder() -> impl Widget<State> {
    Flex::column()
}
