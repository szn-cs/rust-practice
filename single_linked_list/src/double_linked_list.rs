use std::default::Default;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Drop, Fn};
use std::ptr::NonNull;

use datastructure::specification::linked_list::minimal::DoubleLinkedList as Spec;

type Link<T> = NonNull<Node<T>>;

struct Node<T> {
    data: T,
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
}

pub struct DoubleLinkedList<T>
where
    T: Display,
{
    len: usize,
    front: Option<Link<T>>,
    back: Option<Link<T>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> DoubleLinkedList<T>
where
    T: Display,
{
    pub fn new() -> DoubleLinkedList<T> {
        DoubleLinkedList {
            len: 0,
            front: None,
            back: None,
            marker: PhantomData,
        }
    }

    pub fn traverse<F>(&mut self, f: F)
    where
        F: Fn(&mut T) -> (),
    {
        let mut current = self.front;
        while let Some(mut node) = current {
            f(unsafe { &mut node.as_mut().data });
            unsafe {
                println!("{}", (*node.as_ptr()).data);
            }
            current = unsafe { node.as_mut().next };
        }
    }
    // }

    // impl<T> Spec for DoubleLinkedList<T> {
    //     type Data = T;

    pub fn push_front(&mut self, item: T) {
        let new = Box::new(Node {
            data: item,
            next: self.front,
            prev: None,
        });

        let new = NonNull::new(Box::into_raw(new));

        match self.front.take() {
            Some(mut next) => unsafe {
                next.as_mut().prev = new;
            },
            None => {
                self.back = new;
            }
        }

        self.front = new;
        self.len += 1;
    }

    pub fn push_back(&mut self, item: T) {
        let new = Box::new(Node {
            data: item,
            next: None,
            prev: self.back,
        });

        let new = NonNull::new(Box::into_raw(new));

        match self.back.take() {
            None => {
                self.front = new;
            }
            Some(mut prev) => unsafe {
                prev.as_mut().next = new;
            },
        }

        self.back = new;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.front.map(|node| {
            let mut node = unsafe { Box::from_raw(node.as_ptr()) };
            let next = node.next.take();
            match next {
                None => {
                    self.back = None;
                }
                Some(mut next) => unsafe {
                    next.as_mut().prev = None;
                },
            }

            self.front = next;
            self.len -= 1;

            node.data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.map(|node| {
            // len > 0
            let mut node = unsafe { Box::from_raw(node.as_ptr()) };
            let prev = node.prev.take();

            match prev {
                None => {
                    self.front = None;
                }
                Some(mut prev) => unsafe {
                    prev.as_mut().next = None;
                },
            }

            self.back = prev;
            self.len -= 1;

            node.data
        })
    }
}

impl<T> Drop for DoubleLinkedList<T>
where
    T: Display,
{
    fn drop(&mut self) {
        // drop leaked boxed nodes
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>
where
    T: Display,
{
    current: DoubleLinkedList<T>,
}

impl<T> Iterator for IntoIter<T>
where
    T: Display,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.pop_front()
    }
}

impl<T> IntoIterator for DoubleLinkedList<T>
where
    T: Display,
{
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { current: self }
    }
}
