// mod concept_cell;
// mod concept_inherit_mutability;
// mod concept_refcell;

use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

#[derive(Debug, PartialEq, Clone)]
struct Node {
    value: i32,
    next: Option<Link>,
}

impl Node {
    fn new(value: i32) -> Link {
        Rc::new(RefCell::new(Node {
            value,
            next: Option::None,
        }))
    }
}

#[derive(Debug, PartialEq)]
pub struct SingleLinkedList {
    pub len: u64,
    head: Option<Link>,
    tail: Option<Link>,
}

impl SingleLinkedList {
    pub fn new() -> Self {
        SingleLinkedList {
            len: 0,
            head: Option::None,
            tail: Option::None,
        }
    }

    pub fn append(&mut self, value: i32) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Option::Some(new.clone()),
            None => self.head = Option::Some(new.clone()),
        }

        self.tail = Option::Some(new.clone());
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|old| {
            if let Some(n) = old.borrow_mut().next.take() {
                self.head = Option::Some(n);
            } else {
                // last element
                self.tail.take();
            }

            self.len -= 1;
            Rc::try_unwrap(old)
                .ok()
                .expect("No value encountered ! Something went wrong.")
                .into_inner()
                .value
        })
    }
}
