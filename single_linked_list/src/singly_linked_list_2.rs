//! # Sinlgy Linked List:
//!
//! LIFO, head/tail pointers, push/pop O(1)ST, insert at index O(N)ST, traverse O(N)T O(1)S, get length
//!
//! Prevent circular definition which leads to unsized structures. In Rust, use std::rc & std::cell for allowing multiple references to each node (a node maybe refered to through head/tail pionters and other nodes); multiple refernecbes automatically means shared immutable references, thus std::cell would allow circumventing compiler constraints.
//!

use std::cell::RefCell; // allow multiple owners & allocate target on the heap (single-threaded).
use std::iter::{IntoIterator, Iterator};
use std::option::Option;
use std::rc::Rc; // provides interior mutability for encapsulated structures.

type Link = Rc<RefCell<Node>>; // type alias for the pointer solution.

#[derive(Clone, Debug, PartialEq)]
struct Node {
    value: String,
    next: Option<Link>,
}

pub struct SinglyLinkedList {
    pub len: usize,
    head: Option<Link>,
    tail: Option<Link>,
}

impl SinglyLinkedList {
    pub fn new() -> Self {
        SinglyLinkedList {
            len: 0,
            head: Option::None,
            tail: Option::None,
        }
    }

    pub fn push(&mut self, value: String) {
        let node: Link = Node::new(value);

        if let Some(old) = self.tail.take() {
            old.borrow_mut().next = Some(node.clone());
        } else {
            self.head = Some(node.clone());
        }

        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head: Link| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Option::Some(next.clone());
            } else {
                self.tail = Option::None;
            }

            self.len -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is exteremely wrong!")
                .into_inner()
                .value
        })
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }
}

impl Node {
    fn new(value: String) -> Link {
        Rc::new(RefCell::new(Node {
            value,
            next: Option::None,
        }))
    }
}

pub struct ListIterator {
    current: Option<Link>,
}

impl ListIterator {
    fn new(start_at: Option<Link>) -> Self {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Option::None;
        let current = &self.current;

        self.current = match current {
            Some(ref n) => {
                let n = n.borrow();
                result = Some(n.value.clone());
                n.next.clone()
            }
            None => None,
        };

        result
    }
}

impl IntoIterator for SinglyLinkedList {
    type Item = String;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head)
    }
}
