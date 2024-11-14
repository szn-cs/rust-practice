/*
 * Find height of a very large binary tree; i.e. recursive solution would overflow the stack.
 */

use std::ptr::NonNull;

type Link = NonNull<Node>;

#[derive(Debug, PartialEq)]
struct Node {
    left: Option<Link>,
    right: Option<Link>,
}

impl Node {
    pub fn new_with(left: Option<Link>, right: Option<Link>) -> Option<Link> {
        let node = Box::new(Node { left, right });
        NonNull::new(Box::into_raw(node))
    }
}

// Iterative soltion using a stack with node and depth input as input; O(n)T; O(n)S
pub fn depth_binary_tree(node: Option<Link>) -> usize {
    let mut height = 0;
    let mut stack = vec![(node, 1)];

    while let Some((current, depth)) = stack.pop() {
        if current.is_none() {
            continue;
        }

        let current = current.unwrap();
        let current = unsafe { current.as_ref() };

        height = height.max(depth);

        stack.push((current.left, depth + 1));
        stack.push((current.right, depth + 1));
    }

    height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        /*
                         _
                    _           _
                            _      _
                                  _
        */
        let root = Node::new_with(
            Node::new_with(None, None),
            Node::new_with(
                Node::new_with(None, None),
                Node::new_with(Node::new_with(None, None), None),
            ),
        );

        let r = depth_binary_tree(root);

        assert_eq!(r, 4);
    }

    #[test]
    fn case_2() {
        /*
                         _
                              _
                                   _

        */
        let root = Node::new_with(None, Node::new_with(None, Node::new_with(None, None)));

        let r = depth_binary_tree(root);

        assert_eq!(r, 3);
    }
}
