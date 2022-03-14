use std::iter::FromIterator;

pub struct SimpleLinkedList<T: Clone + Copy> {
    head: Option<Box<Node<T>>>,
}

struct Node<T: Clone + Copy> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone + Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut node: &Option<Box<Node<T>>> = &self.head;
        while (*node).is_some() {
            count += 1;
            node = &(node.as_ref().unwrap().next);
        }
        count
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            let mut head = self.head.take().unwrap();
            self.head = head.next.take();
            Some(head.data)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.is_none() {
            true => None,
            false => Some(&self.head.as_ref().unwrap().data),
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut linked_list = SimpleLinkedList::new();

        let mut node: &Option<Box<Node<T>>> = &self.head;
        while (*node).is_some() {
            linked_list.push(node.as_ref().unwrap().data);
            node = &(node.as_ref().unwrap().next);
        }

        linked_list
    }
}

impl<T: Clone + Copy> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T: Clone + Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .fold(SimpleLinkedList::new(), |mut linked_list, element| {
                linked_list.push(element);
                linked_list
            })
    }
}

impl<T: Clone + Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();

        while let Some(element) = linked_list.pop() {
            v.insert(0, element)
        }

        v
    }
}
