mod list_node;

use std::alloc;
use std::ptr;
use list_node::Node;

pub struct List<T> {
    head: *mut Node<T>,
    last: *mut Node<T>,
    size: usize
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: ptr::null_mut(),
            last: ptr::null_mut(),
            size: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn append(&mut self, value: T) {
        let p_node = unsafe {
            Self::allocate(value)
        };
        if self.head.is_null() {
            self.head = p_node;
            self.last = p_node;
        } else {
            unsafe {
                (*(self.last)).next = p_node;
                self.last = p_node;
            }
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) {
        if !self.is_empty() {
            let p_node = self.head;
            unsafe {
                self.head = (*self.head).next;
                let _ = Box::from_raw(p_node);
            }
            self.size -= 1;
        }
    }

    unsafe fn allocate(value: T) -> *mut Node<T> {
        let layout = alloc::Layout::new::<Node<T>>();
        let p_node= alloc::alloc(layout) as *mut Node<T>;
        assert!(!p_node.is_null());
        ptr::write(p_node, Node::new(value));
        p_node
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while !self.is_empty() {
            self.pop_front();
        }
        self.last = ptr::null_mut();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_new() {
        let list = List::<i32>::new();
        assert!(list.is_empty());
        assert!(list.head.is_null());
        assert!(list.last.is_null());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn list_append() {
        let mut list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.len(), 3);
        assert!(!list.head.is_null());
        assert!(!list.last.is_null());
        unsafe {
            assert_eq!(*list.head, Node::new(1));
            assert_eq!((*list.head).value, 1);
            assert_eq!((*(*list.head).next).value, 2);
            assert_eq!((*list.last).value, 3);
        }
    }

    #[test]
    fn list_drop() {
        let mut list = List::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.len(), 3);

        list.pop_front();
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
        list.pop_front();
        assert_eq!(list.len(), 0);

        assert!(list.head.is_null());
    }
}
