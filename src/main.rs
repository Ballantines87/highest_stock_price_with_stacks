struct MaxStack {
    main_stack: Vec<i32>,
    max_stack: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        Self {
            main_stack: vec![],
            max_stack: vec![],
        }
    }

    fn push(&mut self, value: i32) {
        self.main_stack.push(value);
        if !self.max_stack.is_empty() && self.max_stack.last().unwrap() > &value {
            self.max_stack.push(*self.max_stack.last().unwrap());
        }

        else {
            self.max_stack.push(value)
        }
    }

    fn pop(&mut self) {
        self.main_stack.pop();
        self.max_stack.pop();

    }

    fn max_value(&self) -> i32 {
        *self.max_stack.last().unwrap()
    }

}

fn main() {
    let mut stack = MaxStack::new();
    stack.push(55);
    stack.push(80);
    stack.push(120);
    stack.push(99);
    stack.push(22);
    stack.push(140);
    stack.push(145);

    print!("Maximum value of stock : ");
	println!("{:?}", stack.max_value());

    stack.pop();
    print!("Maximum value of stock one week ago : ");
	println!("{:?}", stack.max_value());
}
