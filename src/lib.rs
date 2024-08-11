mod list_node;

use std::ptr;
use list_node::Node;

pub struct List<T> {
    head: *const Node<T>,
    size: usize
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: ptr::null(),
            size: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_new() {
        let list = List::<i32>::new();
    }
}
