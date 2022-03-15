use self::Comparison::*;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> Comparison {
    match (list1.len(), list2.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (x, y) if x > y => {
            if list1.windows(y).any(|l| l == list2) {
                Superlist
            } else {
                Unequal
            }
        }
        (x, y) if x < y => {
            if list2.windows(x).any(|l| l == list1) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if list1 == list2 {
                Equal
            } else {
                Unequal
            }
        }
    }
}
