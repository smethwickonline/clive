use std::fmt::Display;

enum Operation {
    Plus(f64),
    Minus(f64),
    Divide(f64),
    Multiply(f64),
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

struct State {
    value: f64,
    display: String,
    operation: Operation,
}

fn main() {
    let mut state = State {
        value: 12.0,
        display: "0".into(),
        operation: Operation::Minus(10.0),
    };

    println!("{}", state.do_thing());
}
