use std::cmp::Ordering;
use std::ptr;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub next: *mut Node<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: ptr::null_mut(),
        }
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
