use std::ptr;
use crate::List;
use crate::list_node::Node;

pub struct ListIterator<'a, T> {
    list: &'a List<T>,
    curr: *const Node<T>,
    idx: isize,
}

impl<'a, T> ListIterator<'a, T> {
    pub fn new(list: &'a List<T>) -> Self {
        ListIterator {
            list,
            curr: ptr::null(),
            idx: -1,
        }
    }
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

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
            Some(&(*self.curr).value)
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

        let iter = ListIterator::new(&list);
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

        let mut iter = ListIterator::new(&list);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
