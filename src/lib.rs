mod list_node;
mod list_iter;
mod node_iter;

use list_node::Node;
use std::ptr;
use list_iter::ListIterator;
use crate::node_iter::NodeIterator;

pub struct List<T> {
    head: *mut Node<T>,
    last: *mut Node<T>,
    size: usize,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: ptr::null_mut(),
            last: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn front(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&(*self.head).value) }
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.last.is_null() {
            None
        } else {
            unsafe { Some(&(*self.last).value) }
        }
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator::new(self)
    }

    /// Provides a reference to the element at the given index.
    /// Element at index 0 is the front of the sequence.
    /// This operation should compute in O(n) time.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        let mut index: isize = index as isize;
        let mut it = self.iter();
        let mut value = None;
        while index >= 0 {
            value = it.next();
            index -= 1;
        }
        value
    }

    pub fn push_back(&mut self, value: T) {
        let p_node = Box::into_raw(Box::new(Node::new(value)));
        if self.head.is_null() {
            self.head = p_node;
        } else {
            unsafe {
                (*(self.last)).next = p_node;
            }
        }
        self.last = p_node;
        self.size += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let p_node = Box::into_raw(Box::new(Node::new(value)));
        if self.head.is_null() {
            self.last = p_node;
        } else {
            unsafe {
                (*p_node).next = self.head;
            }
        }
        self.head = p_node;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let p_node = self.head;
        unsafe {
            self.head = (*self.head).next;
            self.size -= 1;
            if self.is_empty() {
                self.last = ptr::null_mut();
            }
            Some(Box::from_raw(p_node).value)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let p_node = unsafe {
            Box::from_raw(self.last)
        };
        self.last = self.pre_last();
        self.size -= 1;
        if self.is_empty() {
            self.head = ptr::null_mut();
        }
        Some(p_node.value)
    }

    pub fn append(&mut self, value: T) {
        // let p_node = unsafe { Self::allocate(value) };
        // if self.head.is_null() {
        //     self.head = p_node;
        //     self.last = p_node;
        // } else {
        //     unsafe {
        //         (*(self.last)).next = p_node;
        //         self.last = p_node;
        //     }
        // }
        // self.size += 1;
    }

    fn get_node(&self, index: usize) -> *mut Node<T> {
        assert!(index < self.size);
        let mut index: isize = index as isize;
        let mut it = NodeIterator::new(&self);
        let mut node = ptr::null_mut();
        while index >= 0 {
            node = it.next().unwrap();
            index -= 1;
        }
        node
    }

    fn pre_last(&self) -> *mut Node<T> {
        if self.size > 1 {
            self.get_node(self.size - 2)
        } else {
            ptr::null_mut()
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while !self.is_empty() {
            let p_node = self.head;
            unsafe {
                self.head = (*self.head).next;
                let _ = Box::from_raw(p_node);
            }
            self.size -= 1;
        }
        self.last = ptr::null_mut();
    }
}

// impl<T> Iterator for List<T> {
//     type Item = T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

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
    fn list_front() {
        let mut list = List::<i32>::new();
        assert_eq!(list.front(), None);
        list.push_back(17);
        assert_eq!(list.front(), Some(&17));
        list.push_back(19);
        assert_eq!(list.front(), Some(&17));
    }

    #[test]
    fn list_back() {
        let mut list = List::<i32>::new();
        assert_eq!(list.back(), None);
        list.push_back(17);
        assert_eq!(list.back(), Some(&17));
        list.push_back(19);
        assert_eq!(list.back(), Some(&19));
    }

    #[test]
    fn list_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        for (i, e) in list.iter().enumerate() {
            match i {
                0 => assert_eq!(e, &1),
                1 => assert_eq!(e, &2),
                2 => assert_eq!(e, &3),
                _ => ()
            }
        }
    }

    #[test]
    fn list_get() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn list_push_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert!(!list.head.is_null());
        assert!(!list.last.is_null());
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
        unsafe {
            assert_eq!((*(*list.head).next).value, 2);
        }
    }

    #[test]
    fn list_push_front() {
        let mut list = List::new();

        list.push_front(2);
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(&2));
        assert_eq!(list.back(), Some(&2));

        list.push_back(3);
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&2));
        assert_eq!(list.back(), Some(&3));

        list.push_front(1);
        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
    }

    #[test]
    fn list_pop_front() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&2));
        assert_eq!(list.back(), Some(&3));

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.back(), Some(&3));

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);

        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn list_pop_back() {
        let mut list = List::new();
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&2));

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&1));

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);

        assert_eq!(list.pop_back(), None);
    }


    #[test]
    fn list_drop() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

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
