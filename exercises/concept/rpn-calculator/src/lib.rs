#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    match inputs
        .iter()
        .try_fold(Vec::<i32>::new(), |mut stack, input| {
            if let Some(new) = match input {
                Value(value) => Some(*value),
                Add => stack.pop().and_then(|n2| stack.pop().map(|n1| n1 + n2)),
                Subtract => stack.pop().and_then(|n2| stack.pop().map(|n1| n1 - n2)),
                Multiply => stack.pop().and_then(|n2| stack.pop().map(|n1| n1 * n2)),
                Divide => stack.pop().and_then(|n2| stack.pop().map(|n1| n1 / n2)),
            } {
                stack.push(new);
                Some(stack)
            } else {
                None
            }
        }) {
        Some(mut stack) => stack.pop().and_then(|v| match stack.is_empty() {
            true => Some(v),
            false => None,
        }),
        None => None,
    }
}
