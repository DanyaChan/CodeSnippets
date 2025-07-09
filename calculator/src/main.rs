use std::collections::HashMap;

struct Evaluator {
    variable_values: HashMap<String, i32>,
}

impl Evaluator {
    fn get_priority(op: u8) -> i32 {
        match op {
            b'+' => 0,
            b'-' => 0,
            b'*' => 1,
            b'/' => 1,
            _ => panic!("Wrong operator {op}"),
        }
    }
    fn evaluate_op(&self, value_stack: &mut Vec<i32>, op: u8) {
        let a = value_stack.pop().unwrap();
        let b = value_stack.pop().unwrap();
        let res = match op {
            b'+' => a + b,
            b'*' => a * b,
            b'/' => a / b,
            b'-' => a - b,
            _ => panic!("Wrong operator {op}"),
        };
        value_stack.push(res);
    }
    fn apply_value(
        &self,
        value_stack: &mut Vec<i32>,
        cur_var: &mut Vec<u8>,
        cur_number: &mut Vec<u8>,
    ) {
        if cur_number.is_empty() && cur_var.is_empty() {
            panic!("Expected value or var")
        }
        if !cur_number.is_empty() && !cur_var.is_empty() {
            panic!("Wrong var format only a..z allowed")
        }
        value_stack.push(if cur_number.is_empty() {
            *self
                .variable_values
                .get(&String::from_utf8(cur_var.clone()).expect("Wrong var format"))
                .expect(
                    ("No such variable".to_owned() + &String::from_utf8(cur_var.clone()).unwrap())
                        .as_str(),
                )
        } else {
            String::from_utf8(cur_number.clone())
                .expect("Wrong value format")
                .parse::<i32>()
                .expect("Wrong number format")
        });
        cur_var.clear();
        cur_number.clear();
    }
    fn evaluate(&self, expression: &str) -> i32 {
        let mut value_stack: Vec<i32> = vec![];
        let mut operator_stack: Vec<u8> = vec![];
        let mut cur_var: Vec<u8> = vec![];
        let mut cur_number: Vec<u8> = vec![];
        for c in expression.bytes() {
            let dbug_char = (c as char).to_string();
            match c {
                b'0'..b'9' => cur_number.push(c),
                b'a'..b'z' => cur_var.push(c),
                b'(' => operator_stack.push(c),
                b')' => {
                    self.apply_value(&mut value_stack, &mut cur_var, &mut cur_number);
                    loop {
                        let op = operator_stack.pop().expect("Expected operator");
                        self.evaluate_op(&mut value_stack, op);
                        if operator_stack.is_empty() {
                            panic!("Expected (")
                        }
                        if *operator_stack.last().unwrap() == b'(' {
                            operator_stack.pop();
                            break;
                        }
                    }
                }
                b'*' | b'+' | b'-' | b'/' => {
                    if !cur_var.is_empty() || !cur_number.is_empty() {
                        self.apply_value(&mut value_stack, &mut cur_var, &mut cur_number);
                    }
                    if operator_stack.is_empty() || *operator_stack.last().unwrap() == b'(' {
                        operator_stack.push(c);
                    } else {
                        let last_op = operator_stack.last().unwrap();
                        if Self::get_priority(*last_op) >= Self::get_priority(c) {
                            self.evaluate_op(&mut value_stack, operator_stack.pop().unwrap());
                        }
                        operator_stack.push(c);
                    }
                }
                b' ' => {}
                _ => panic!("wrong symbol"),
            }
        }
        if !cur_var.is_empty() || !cur_number.is_empty() {
            self.apply_value(&mut value_stack, &mut cur_var, &mut cur_number);
        }
        while !operator_stack.is_empty() {
            let op = operator_stack.pop().unwrap();
            match op {
                b'*' | b'+' | b'-' | b'/' => {
                    self.evaluate_op(&mut value_stack, op);
                }
                _ => {
                    panic!("Unxepected operator \"{op}\" in stack");
                }
            }
        }
        return value_stack.pop().expect("Expected value");
    }
    fn add_var(&mut self, name: &str, value: i32) {
        self.variable_values.insert(name.to_string(), value);
    }
}

fn main() {
    let mut eval = Evaluator {
        variable_values: HashMap::new(),
    };
    eval.add_var("a", 3);
    eval.add_var("b", 2);
    print!("result {}", eval.evaluate("(a+b)*4"));
}
