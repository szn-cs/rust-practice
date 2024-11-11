use std::cmp::max;
/**
 * check if tree is balanced (right/left height differ by at most 1)
 */
use std::ptr::NonNull;

type Link = Option<NonNull<Node>>;

struct Node {
    // value: (),
    left: Link,
    right: Link,
}

impl Node {
    pub fn new() -> Link {
        NonNull::new(Box::into_raw(Box::new(Node {
            left: None,
            right: None,
        })))
    }

    pub fn with_children(left: Link, right: Link) -> Link {
        NonNull::new(Box::into_raw(Box::new(Node { left, right })))
    }
}

pub fn is_balanced_tree(n: NonNull<Node>) -> bool {
    is_balanced_tree_helper(n).0
}

pub fn is_balanced_tree_helper(n: NonNull<Node>) -> (bool, usize) {
    let height;
    let valid;
    let n = unsafe { n.as_ref() };

    // traverse subtrees
    let l = if let Some(left) = n.left {
        is_balanced_tree_helper(left)
    } else {
        (true, 0)
    };

    let r = if let Some(right) = n.right {
        is_balanced_tree_helper(right)
    } else {
        (true, 0)
    };

    let diff = l.1 as i32 - r.1 as i32;
    valid = l.0 && r.0 && i32::abs(diff) <= 1;
    height = max(l.1, r.1) + 1;

    (valid, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        /*
         *              _
         *          _       _
         *        _   _
         *
         */
        let root = Node::with_children(
            Node::with_children(Node::new(), Node::new()),
            Node::with_children(None, None),
        );

        let r = is_balanced_tree(root.unwrap());
        assert_eq!(r, true);
    }

    #[test]
    fn case_2() {
        /*
         *              _
         *          _
         *        _
         *
         */
        let root = Node::with_children(Node::with_children(Node::new(), None), None);

        let r = is_balanced_tree(root.unwrap());
        assert_eq!(r, false);
    }
}
