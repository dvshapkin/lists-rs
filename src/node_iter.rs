use std::ptr;
use crate::List;
use crate::list_node::Node;

pub struct NodeIterator<'a, T> {
    list: &'a List<T>,
    curr: *mut Node<T>,
    idx: isize,
}

impl<'a, T> NodeIterator<'a, T> {
    pub fn new(list: &'a List<T>) -> Self {
        NodeIterator {
            list,
            curr: ptr::null_mut(),
            idx: -1,
        }
    }
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = *mut Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.is_empty() {
            return None;
        } else if self.curr == self.list.last {
            return None;
        }
        self.idx += 1;
        unsafe {
            if self.curr.is_null() {
                self.curr = self.list.first;
            } else {
                self.curr = (*self.curr).next;
            }
            Some(self.curr)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_new() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let iter = NodeIterator::new(&list);
        let mut count = 0;
        for node in iter {
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn iter_next() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = NodeIterator::new(&list);
        unsafe {
            assert_eq!((*iter.next().unwrap()).value, 1);
            assert_eq!((*iter.next().unwrap()).value, 2);
            assert_eq!((*iter.next().unwrap()).value, 3);
        }
    }
}
