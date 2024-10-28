use std::ptr::NonNull;
use std::string::ToString;

type Link = Option<NonNull<Node>>;

struct Node {
    value: char,
    next: Link,
}

impl Node {
    pub fn new(c: char) -> Link {
        NonNull::new(Box::into_raw(Box::new(Node {
            value: c,
            next: None,
        })))
    }

    pub fn from_vec(v: &[char]) -> Link {
        let mut head = Node::new('z');
        let mut current = head;
        for &c in v {
            let n = Node::new(c);
            let c = unsafe { current.unwrap().as_mut() };
            c.next = n;
            current = c.next;
        }

        let head = unsafe { head.unwrap().as_mut() };
        head.next
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut s = String::from(self.value);
        let mut current = self.next;
        while let Some(c) = current {
            let c = unsafe { c.as_ref() };
            s.push(' ');
            s.push(c.value);
            current = c.next;
        }
        s.trim();
        s
    }
}

/**
 * calculate lengths then advance longer list to be parallel to the other list; compare elements till they meet.
 * O(max(n, m))T; O(1)S;
 */
mod impl_1 {
    use super::*;

    fn get_len(l: Link) -> usize {
        let mut len = 0;
        let mut current = l;
        while let Some(mut c) = current {
            len += 1;
            let c = unsafe { c.as_mut() };
            current = c.next;
        }
        len
    }

    pub fn linked_list_intersection(l1: Link, l2: Link) -> Link {
        let (len1, len2) = (get_len(l1), get_len(l2));
        if len1 == 0 || len2 == 0 {
            return None;
        }

        let (mut p1, mut p2) = (l1, l2);
        if len1 > len2 {
            let advance = len1 - len2;
            for _ in 1..=advance {
                p1 = unsafe { p1.unwrap().as_mut().next };
            }
        } else {
            let advance = len2 - len1;
            for _ in 1..=advance {
                p2 = unsafe { p2.unwrap().as_mut().next };
            }
        }

        // both pointers are same distance from end
        let mut c1 = p1;
        let mut c2 = p2;
        while let Some(_) = c1 {
            let c1_ref = unsafe { c1.unwrap().as_mut() };
            let c2_ref = unsafe { c2.unwrap().as_mut() };

            if c1_ref.value == c2_ref.value {
                break;
            }

            c1 = c1_ref.next;
            c2 = c2_ref.next;
        }

        c1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = Node::from_vec(&['a', 'b', 'c', 'd']);
        let l2 = Node::from_vec(&['t']);
        // link intersection
        unsafe {
            let mut intersection = l1;
            for _ in 1..=2 {
                let c = intersection.unwrap().as_mut();
                intersection = c.next;
            }
            l2.unwrap().as_mut().next = intersection;
        }

        // let (s1, s2) = {
        //     let l1 = unsafe { l1.unwrap().as_mut() };
        //     let l2 = unsafe { l2.unwrap().as_mut() };
        //     (l1.to_string(), l2.to_string())
        // };
        // println!("l1 {}\nl2 {}", s1, s2);

        let intersection = impl_1::linked_list_intersection(l1, l2);
        let value = unsafe { intersection.unwrap().as_mut().value };

        assert_eq!(value, 'c');
    }
}
