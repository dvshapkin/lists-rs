use crate::common::HasChild;
use std::cmp::Ordering;
use std::ptr;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    next: *const Node<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            next: ptr::null(),
        }
    }
}

impl<T> HasChild for Node<T> {
    fn add_child(&mut self, other: Self) {
        self.next = &other;
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value < other.value {
            Some(Ordering::Less)
        } else if self.value > other.value {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn node_new() {
        let node = Node::new(777);

        assert_eq!(node.value, 777);
        assert!(node.next.is_null());
    }

    #[test]
    fn node_add_child() {
        let mut first = Node::new("first");
        let second = Node::new("second");

        assert_eq!(first.value, "first");
        assert_eq!(second.value, "second");
        assert_ne!(first, second);

        let second_addr = &second as *const Node<&str>;
        first.add_child(second);

        assert_eq!(first.next, second_addr);
        unsafe {
            assert_eq!(*first.next, *second_addr);
        }
    }

    #[test]
    fn node_cmp() {
        let first = Node::new(1);
        let second = Node::new(2);

        assert!(first < second);
        assert!(first <= second);
        assert!(!(first > second));
        assert!(!(first >= second));
        assert_ne!(first, second);

        let first = Node::new(3);
        let second = first.clone();

        assert!(!(first < second));
        assert!(!(first > second));
        assert!(first <= second);
        assert!(first >= second);
        assert_eq!(first, second);
    }
}
