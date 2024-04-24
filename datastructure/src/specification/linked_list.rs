use std::iter::{IntoIterator, Iterator};
use std::ops::Fn;
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
    fn insert_before_head(&mut self, data: Self::Data, position: usize);
    fn push(&mut self, data: Self::Data); // insert to head

    // deletion
    fn remove_tail(&mut self, position: usize) -> Option<Self::Data>;
    fn pop(&mut self) -> Option<Self::Data>; // delete from tail

    // traversal
    // fn into_iter(self) -> Self::IntoIter /* Iterator<Iterm> */; // provides external iteration; used for consuming, reference, mutation
    fn traverse(&self, f: impl Fn(&Self::Data)); // internal iteration
}

pub mod minimal {
    pub trait SingleLinkedList: IntoIterator {
        type Data;

        // fields: head, len
        //         (No tail pointer)

        fn push(&mut self, data: Self::Data);
        fn pop(&mut self) -> Option<Self::Data>;
    }
}
