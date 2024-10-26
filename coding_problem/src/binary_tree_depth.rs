use std::cmp::max;
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

// DFS recursive with depth parameter
mod impl_1 {
    use super::*;

    pub fn deepest(tree: NonNull<Node>) -> usize {
        deepest_helper(Some(tree)) - 1 // depth of root is 0
    }

    fn deepest_helper(tree: Link) -> usize {
        if let None = tree {
            return 0;
        }

        let n = unsafe { tree.unwrap().as_mut() };

        1 + max(deepest_helper(n.left), deepest_helper(n.right))
    }
}

// DFS non-recursive
mod impl_2 {
    use super::*;

    pub fn deepest(tree: NonNull<Node>) -> usize {
        let mut s: Vec<(NonNull<Node>, usize)> = vec![(tree, 0)]; // (ptr, depth)

        let mut max_depth = 0;

        // DFS
        while let Some((n, d)) = s.pop() {
            let n = unsafe { n.as_ref() };

            max_depth = max_depth.max(d);

            if let Some(c) = n.left {
                s.push((c, d + 1));
            }

            if let Some(c) = n.right {
                s.push((c, d + 1));
            }
        }

        max_depth
    }
}

// BFS non-recursive
mod impl_3 {
    use super::*;
    use std::collections::VecDeque;

    pub fn deepest(tree: NonNull<Node>) -> usize {
        let mut s: VecDeque<(NonNull<Node>, usize)> = VecDeque::from([(tree, 0)]); // (ptr, depth)

        let mut max_depth = 0;

        // DFS
        while let Some((n, d)) = s.pop_front() {
            let n = unsafe { n.as_ref() };

            max_depth = max_depth.max(d);

            if let Some(c) = n.left {
                s.push_back((c, d + 1));
            }

            if let Some(c) = n.right {
                s.push_back((c, d + 1));
            }
        }

        max_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_impl_1() {
        use impl_1::deepest;

        /*          depth = 4 (# of edges)
         *              _
         *          _       _
         *        _   _    _
         *                  _
         *                   _
         */
        let root = Node::with_children(
            Node::with_children(Node::new(), Node::new()),
            Node::with_children(
                Node::with_children(None, Node::with_children(None, Node::new())),
                None,
            ),
        );

        assert_eq!(deepest(root.unwrap()), 4);
    }

    #[test]
    fn case_impl_2() {
        use impl_2::deepest;

        /*          depth = 4 (# of edges)
         *              _
         *          _       _
         *        _   _    _
         *                  _
         *                   _
         */
        let root = Node::with_children(
            Node::with_children(Node::new(), Node::new()),
            Node::with_children(
                Node::with_children(None, Node::with_children(None, Node::new())),
                None,
            ),
        );

        assert_eq!(deepest(root.unwrap()), 4);
    }

    #[test]
    fn case_impl_3() {
        use impl_3::deepest;

        /*          depth = 4 (# of edges)
         *              _
         *          _       _
         *        _   _    _
         *                  _
         *                   _
         */
        let root = Node::with_children(
            Node::with_children(Node::new(), Node::new()),
            Node::with_children(
                Node::with_children(None, Node::with_children(None, Node::new())),
                None,
            ),
        );

        assert_eq!(deepest(root.unwrap()), 4);
    }
}
