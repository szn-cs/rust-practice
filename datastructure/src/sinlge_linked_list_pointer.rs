use std::boxed::Box;
use std::iter::IntoIterator;
use std::marker::PhantomData;
use std::option::Option;
use std::ptr::NonNull;

use specification::datastructure::linked_list::minimal::SingleLinkedList as Spec;

/**
 * - implements Copy
 * - points leaked Box requiring manual memory management
 */
type Link<T> = NonNull<Node<T>>;

pub struct Node<T> {
    data: T,
    next: Option<Link<T>>,
}

// Implementation using head pointer only
pub struct SingleLinkedList<T> {
    pub len: usize,
    head: Option<Link<T>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            marker: PhantomData,
        }
    }
}

impl<T> Spec for SingleLinkedList<T> {
    type Data = T;

    fn push(&mut self, data: Self::Data) {
        let mut node = Box::new(Node { data, next: None });

        node.next = self.head;

        self.head = NonNull::new(Box::into_raw(node));
        self.len += 1;
    }

    fn pop(&mut self) -> Option<Self::Data> {
        self.head.map(|mut head| {
            let mut current = unsafe { head.as_mut() };
            let mut prev = None;

            while let Some(mut next) = current.next {
                let next = unsafe { next.as_mut() };

                prev = Some(current);
                current = next;
            }

            if let Some(prev) = prev {
                (*prev).next = None;
            } else {
                self.head = None;
            }

            self.len -= 1;

            let tail = unsafe { Box::from_raw(current) };
            tail.data
            // tail box out-of-scope is dropped
        })
    }
}

//Iterator
pub struct Iter<'a, T> {
    current: Option<Link<T>>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|mut node| {
            let node = unsafe { node.as_mut() };
            self.current = node.next;

            &mut node.data
        })
    }
}

impl<'a, T> IntoIterator for &'a mut SingleLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            current: self.head,
            marker: PhantomData,
        }
    }
}

// IntoIterator
pub struct IntoIter<T> {
    list: SingleLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T> IntoIterator for SingleLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::<Self::Item> { list: self }
    }
}

impl<T> Drop for SingleLinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
