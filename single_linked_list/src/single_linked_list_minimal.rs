use std::cell::{Ref, RefCell, RefMut};
use std::clone::Clone;
use std::iter::{IntoIterator, Iterator};
use std::rc::Rc;

use specification::datastructure::linked_list::minimal::SingleLinkedList as Spec;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct SingleLinkedList<T>
where
    T: Clone,
{
    head: Link<T>,
    tail: Link<T>,
    pub len: usize,
}

impl<T: Clone> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /*
    // NOTE: relying on std::rc::Rc<std::cell::RefCell<_>> Rust safe API is cumbersome and limited for the purpose of returning internal references data within RefCell; This approach is not well fitted for creating APIs that provide interior mutability to the users of the library.

    pub fn traverse(&mut self, f: impl Fn(&T)) {
        let mut c = self.head.map(|t| t.as_ref());
        while let Some(ref node) = c {
            f(&node.borrow().value);
            c = c.map(|t| Ref::map(t.borrow(), |i| i.next.clone()));
        }
    }
    */
}

impl<T> Spec for SingleLinkedList<T>
where
    T: Clone,
{
    type Data = T;

    fn push(&mut self, value: Self::Data) {
        let n = Rc::new(RefCell::new(Node { value, next: None }));

        match self.tail.take() {
            Some(t) => {
                t.borrow_mut().next = Some(n.clone());
            }
            None => self.head = Some(n.clone()),
        }

        self.tail = Some(n);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<Self::Data> {
        let n = self.head.take();

        n.map(|n| {
            if let Some(h) = n.borrow_mut().next.take() {
                self.head = Some(h.clone());
            } else {
                self.tail.take();
            }

            self.len -= 1;

            Rc::try_unwrap(n)
                .ok()
                .expect("option must be occupied with value")
                .into_inner()
                .value
        })
    }
}

impl<T: Clone> IntoIterator for SingleLinkedList<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator {
            current: self.head.clone(),
        }
    }
}

pub struct ListIterator<T>
where
    T: Clone,
{
    current: Link<T>,
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(n) => {
                self.current = if let Some(ref n) = n.borrow().next {
                    Some(n.clone())
                } else {
                    None
                };

                Some(n.borrow().value.clone())
            }
        }
    }
}
