/**
 * count number of univalue trees in any level of the tree;
 * univalue subtree is a tree that left and right form a unival subtree of same type.
 */
use std::ptr::NonNull;

type Link = NonNull<Node>;

#[derive(PartialEq, Debug, Clone)]
enum Value {
    X,
    Y,
}

#[derive(Debug)]
struct Node {
    value: Value,
    left: Option<Link>,
    right: Option<Link>,
}

impl Node {
    pub fn new(value: Value) -> Option<Link> {
        let n = Box::new(Node {
            value,
            left: None,
            right: None,
        });

        NonNull::new(Box::into_raw(n))
    }

    pub fn new_with(value: Value, left: Option<Link>, right: Option<Link>) -> Option<Link> {
        let n = Box::new(Node { value, left, right });
        NonNull::new(Box::into_raw(n))
    }
}

// recursive count valid subtrees; O(n)T; O(n)S
pub fn count_unival_subtrees(node: Option<Link>) -> usize {
    let (counter, _) = count_unival_subtrees_helper(node);

    counter
}

pub fn count_unival_subtrees_helper(node: Option<Link>) -> (usize, bool) {
    // base case
    if node.is_none() {
        return (0, true);
    }

    let node_ptr = node.unwrap();
    let node_ref = unsafe { node_ptr.as_ref() };

    let l = count_unival_subtrees_helper(node_ref.left);
    let r = count_unival_subtrees_helper(node_ref.right);

    // check univalue
    let mut is_unival = true;
    let mut counter = l.0 + r.0;

    if let Some(left) = node_ref.left {
        let left = unsafe { left.as_ref() };
        if node_ref.value != left.value {
            is_unival = false;
        }
    }

    if let Some(right) = node_ref.right {
        let right = unsafe { right.as_ref() };
        if node_ref.value != right.value {
            is_unival = false;
        }
    }

    if !l.1 || !r.1 {
        is_unival = false;
    }

    if is_unival {
        counter += 1;
    }

    (counter, is_unival)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Value::*;

    #[test]
    fn case_1() {
        /*
         *          X
         *      Y       X
         *    X       Y    Y
         *                Y  Y
         */

        let root = Node::new_with(
            X,
            Node::new_with(Y, Node::new(X), None),
            Node::new_with(
                X,
                Node::new(Y),
                Node::new_with(Y, Node::new(Y), Node::new(Y)),
            ),
        );

        let r = count_unival_subtrees(root);

        assert_eq!(r, 5);
    }
}
