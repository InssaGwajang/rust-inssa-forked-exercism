use self::CalculatorInput::*;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            Value(value) => stack.push(*value),
            Add => {
                if stack.len() >= 2 {
                    let n2 = stack.pop().unwrap();
                    let n1 = stack.pop().unwrap();

                    stack.push(n1 + n2);
                } else {
                    return None;
                }
            }
            Subtract => {
                if stack.len() >= 2 {
                    let n2 = stack.pop().unwrap();
                    let n1 = stack.pop().unwrap();

                    stack.push(n1 - n2);
                } else {
                    return None;
                }
            }
            Multiply => {
                if stack.len() >= 2 {
                    let n2 = stack.pop().unwrap();
                    let n1 = stack.pop().unwrap();

                    stack.push(n1 * n2);
                } else {
                    return None;
                }
            }
            Divide => {
                if stack.len() >= 2 {
                    let n2 = stack.pop().unwrap();
                    let n1 = stack.pop().unwrap();

                    stack.push(n1 / n2);
                } else {
                    return None;
                }
            }
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }

    // stack
    //     .pop()
    //     .and_then(|result| if stack.is_empty() { Some(result) } else { None })
}
