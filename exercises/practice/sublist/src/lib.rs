#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> Comparison {
    use Comparison::*;

    match (list1.len(), list2.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (x, y) if x > y => {
            if list1.windows(y).any(|v| v == list2) {
                Superlist
            } else {
                Unequal
            }
        }
        (x, y) if x < y => {
            if list2.windows(x).any(|v| v == list1) {
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

// fn is_sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> bool {
//     for i in 0..=list2.len() - list1.len() {
//         if *list1 == list2[i..i + list1.len()] {
//             return true;
//         }
//     }

//     false
// }

// pub fn sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> Comparison {
//     match (list1.len(), list2.len()) {
//         (x, y) if x < y => {
//             if is_sublist(list1, list2) {
//                 Comparison::Sublist
//             } else {
//                 Comparison::Unequal
//             }
//         }
//         (x, y) if x > y => {
//             if is_sublist(list2, list1) {
//                 Comparison::Superlist
//             } else {
//                 Comparison::Unequal
//             }
//         }
//         (_, _) => {
//             if list1 == list2 {
//                 Comparison::Equal
//             } else {
//                 Comparison::Unequal
//             }
//         }
//     }
// }
