// prblem: given multiple sorted linked lists, merge them efficiently into a single linked list

use std::boxed::Box;
use std::cmp::Ordering;
use std::ptr::NonNull;
use std::string::ToString;

type Link = NonNull<Node>;

struct Node {
    value: i32,
    next: Option<Link>,
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut s = String::from(format!("{} ", self.value));
        let mut current = self.next;
        while let Some(mut current_ptr) = current {
            let current_ref = unsafe { current_ptr.as_mut() };
            s.push_str(&format!("{} ", unsafe { current_ref.value }));
            current = current_ref.next;
        }
        s.trim().to_owned()
    }
}

// BF approach: O((n+m)*log(n+m))T; O(n+m)S
mod impl_1 {
    use super::*;

    pub fn merge_linked_list(v: Vec<Link>) -> Box<Node> {
        let mut numbers: Vec<i32> = vec![];

        for l in v {
            let mut current = Some(l);
            while let Some(mut current_ptr) = current {
                let current_ref = unsafe { current_ptr.as_mut() };
                numbers.push(current_ref.value);
                current = current_ref.next;
            }
        }

        // sort elements
        numbers.sort();

        // construct linked list
        let mut dummy = NonNull::new(Box::into_raw(Box::new(Node {
            value: i32::MIN,
            next: None,
        })));

        let mut current = dummy;
        for e in numbers {
            let mut node = NonNull::new(Box::into_raw(Box::new(Node {
                value: e,
                next: None,
            })));
            let c_ref = unsafe { current.unwrap().as_mut() };
            c_ref.next = node;
            current = node;
        }

        let head = (unsafe { dummy.unwrap().as_mut() }).next;
        unsafe { Box::from_raw(head.unwrap().as_ptr()) }
    }
}

mod impl_2 {
    use super::*;

    pub fn merge_linked_list(mut v: Vec<Option<Link>>) -> Option<Box<Node>> {
        if v.is_empty() {
            return None;
        }

        // new list head
        let mut dummy = NonNull::new(Box::into_raw(Box::new(Node {
            value: i32::MAX,
            next: None,
        })));
        let mut current = dummy;

        // compare lists and extract minimum until no elements left
        loop {
            let mut filtered_iter = v
                .iter_mut()
                .enumerate()
                .filter(|(_, n)| n.is_some())
                .peekable();
            if filtered_iter.peek().is_none() {
                break;
            }

            let (min_index, _) = filtered_iter.fold((0, i32::MAX), |mut acc, (i, &mut c)| {
                let current_ref = unsafe { c.unwrap().as_mut() };
                if let Ordering::Greater | Ordering::Equal = acc.1.cmp(&current_ref.value) {
                    acc = (i, current_ref.value);
                }
                acc
            });

            let current_ref = unsafe { current.unwrap().as_mut() };
            let min_ptr = v[min_index];
            let min_ref = unsafe { min_ptr.unwrap().as_mut() };

            v[min_index] = min_ref.next;

            min_ref.next = None;
            current_ref.next = min_ptr;
            current = min_ptr;
        }

        let dummy_ref = unsafe { dummy.unwrap().as_mut() };
        dummy_ref.next.map(|n| unsafe { Box::from_raw(n.as_ptr()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn case_1() {
        use impl_1::merge_linked_list;

        let l1 = {
            let mut dummy = NonNull::new(Box::into_raw(Box::new(Node {
                value: i32::MIN,
                next: None,
            })))
            .unwrap();

            let mut current = dummy;
            for (i, &value) in [3, 5, 6, 10, 11, 13].iter().enumerate() {
                let e = NonNull::new(Box::into_raw(Box::new(Node { value, next: None })));
                unsafe {
                    current.as_mut().next = e;
                };
                current = e.unwrap();
            }

            unsafe { dummy.as_mut().next }
        };

        let l2 = {
            let mut dummy = NonNull::new(Box::into_raw(Box::new(Node {
                value: i32::MIN,
                next: None,
            })));
            let mut current = dummy.unwrap();
            for &value in [2, 4, 11, 12].iter() {
                let e = NonNull::new(Box::into_raw(Box::new(Node { value, next: None })));
                let c = unsafe { current.as_mut() };
                c.next = e;
                current = e.unwrap();
            }
            let d = unsafe { dummy.unwrap().as_mut() };
            d.next
        };

        // println!("{}", {
        //     let n1 = unsafe { l1.unwrap().as_mut() };
        //     let n2 = unsafe { l2.unwrap().as_mut() };
        //     {
        //         let mut s = n1.to_string();
        //         s.push('\n');
        //         s.push_str(&n2.to_string());
        //         s
        //     }
        // });

        let r = merge_linked_list(vec![l1.unwrap(), l2.unwrap()]);
        assert_eq!(r.to_string(), "2 3 4 5 6 10 11 11 12 13");
    }

    #[test]
    fn case_2() {
        use impl_2::merge_linked_list;

        let l1 = {
            let mut dummy = NonNull::new(Box::into_raw(Box::new(Node {
                value: i32::MIN,
                next: None,
            })))
            .unwrap();

            let mut current = dummy;
            for (i, &value) in [3, 5, 6, 10, 11, 13].iter().enumerate() {
                let e = NonNull::new(Box::into_raw(Box::new(Node { value, next: None })));
                unsafe {
                    current.as_mut().next = e;
                };
                current = e.unwrap();
            }

            unsafe { dummy.as_mut().next }
        };

        let l2 = {
            let mut dummy = NonNull::new(Box::into_raw(Box::new(Node {
                value: i32::MIN,
                next: None,
            })));
            let mut current = dummy.unwrap();
            for &value in [2, 4, 11, 12].iter() {
                let e = NonNull::new(Box::into_raw(Box::new(Node { value, next: None })));
                let c = unsafe { current.as_mut() };
                c.next = e;
                current = e.unwrap();
            }
            let d = unsafe { dummy.unwrap().as_mut() };
            d.next
        };

        // println!("{}", {
        //     let n1 = unsafe { l1.unwrap().as_mut() };
        //     let n2 = unsafe { l2.unwrap().as_mut() };
        //     {
        //         let mut s = n1.to_string();
        //         s.push('\n');
        //         s.push_str(&n2.to_string());
        //         s
        //     }
        // });

        let r = merge_linked_list(vec![l1, l2]);
        assert_eq!(r.unwrap().to_string(), "2 3 4 5 6 10 11 11 12 13");
    }
}
