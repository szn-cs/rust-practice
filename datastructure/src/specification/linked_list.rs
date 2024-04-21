use std::iter::{IntoIterator, Iterator};
use std::option::Option;

/**
 * Default spec of singly linked list:
 * - FIFO insertion & retrival ordering
 * - head pointer without tail pointer
 */
pub trait SingleLinkedList: IntoIterator {
    type Data; // data value type

    // fields: head, tail, len;

    // insertion
    fn insert(&mut self, data: Self::Data, position: usize);
    fn push(&mut self, data: Self::Data); // insert to head

    // deletion
    fn remove(&mut self, position: usize) -> Option<Self::Data>;
    fn pop(&mut self) -> Option<Self::Data>; // delete from tail

    // traversal
    // fn into_iter(self) -> Self::IntoIter /* Iterator<Iterm> */; // used for consuming, reference, mutation
}

pub mod minimal {
    pub trait SingleLinkedList: IntoIterator {
        type Data;

        // fields: head, len

        fn push(&mut self, data: Self::Data);
        fn pop(&mut self);
    }
}
