// problem: remove consecutive nodes from linked list that sum up to zero

use std::boxed::Box;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::string::ToString;

// linekdlist structure
struct Node {
    value: i32,
    next: Option<NonNull<Node>>,
    _marker: PhantomData<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            next: None,
            _marker: PhantomData,
        })
    }

    pub fn from_slice(l: &[i32]) -> Box<Node> {
        let n = Node::new(l[0]);
        let head = NonNull::new(Box::into_raw(n));
        let mut current = head.clone();

        for &e in l.iter().skip(1) {
            let n = NonNull::new(Box::into_raw(Node::new(e)));
            unsafe {
                current.unwrap().as_mut().next = n;
            }

            current = n;
        }

        unsafe { Box::from_raw(head.unwrap().as_ptr()) }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut s: String = String::new();

        s += &format!(" {}", self.value);
        let mut current = self.next;

        while let Some(c) = current {
            let c = unsafe { c.as_ref() };
            s += &format!(" {}", c.value);
            current = c.next;
        }

        s.trim().into()
    }
}

struct OrderedMap {
    vector: Vec<i32>,
    map: HashMap<i32, NonNull<Node>>,
    len: usize,
}

impl OrderedMap {
    pub fn new() -> Self {
        OrderedMap {
            vector: vec![],
            map: HashMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, k: i32, v: NonNull<Node>) {
        self.vector.push(k);
        self.map.insert(k, v);
        self.len += 1;
    }

    pub fn get(&self, k: i32) -> Option<NonNull<Node>> {
        self.map.get(&k).map(|&ptr| ptr)
    }

    pub fn pop(&mut self) -> Option<NonNull<Node>> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        let last = self.vector.pop().unwrap();
        Some(
            *self
                .map
                .get(&last)
                .expect("corresponding key of self.vector must be present in self.map"),
        )
    }

    pub fn peak(&self) -> Option<i32> {
        if self.len > 0 {
            Some(self.vector[self.vector.len() - 1])
        } else {
            None
        }
    }
}

fn remove_zero_sum_nodes(l: Box<Node>) -> Option<Box<Node>> {
    let mut head: Option<NonNull<Node>> = NonNull::new(Box::into_raw(l));
    let mut ordered_map = OrderedMap::new();
    let mut sum: i32 = 0;

    let mut current = head.clone();
    while let Some(mut current_ptr) = current {
        let current_ptr = unsafe { current_ptr.as_mut() };
        sum += current_ptr.value;

        let prev = ordered_map.get(sum);

        let mut pop_inbetween = || {
            while let Some(last) = ordered_map.peak() {
                if last == sum {
                    break;
                }

                let last = unsafe { ordered_map.pop().unwrap().as_mut() };
                last.next = None;
                let b = unsafe { Box::from_raw(last) }; // drop allocated memory
            }
        };

        if sum == 0 && prev.is_none() {
            head = current_ptr.next;
            pop_inbetween();
        } else if let Some(mut prev_ptr) = prev {
            let prev_ptr = unsafe { prev_ptr.as_mut() };
            prev_ptr.next = current_ptr.next;
            pop_inbetween();
        } else {
            ordered_map.insert(sum, current.unwrap());
        }

        current = current_ptr.next;
    }

    head.map(|ptr| unsafe { Box::from_raw(ptr.as_ptr()) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn case_1() {
        let head = Node::from_slice(&[5, 4, 1, -2, 2, -1, -4, 7]);
        let result = remove_zero_sum_nodes(head).unwrap();
        assert_eq!(result.to_string(), "5 7");
    }
}
