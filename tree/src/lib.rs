/**
 * - Binary tree datastructure implementation.
 * - Traversal implementation: (preorder, inorder, postorder),
 */

/**
 * Indecies
 */
pub mod impl_1 {
    pub struct Tree<T> {
        list: Vec<Node<T>>,
        first_free_slot: Option<usize>,
        last_free_slot: Option<usize>,
    }

    pub struct Node<T> {
        index: usize,
        value: T,
        parent: usize,
        children: Vec<usize>,
    }
}

/**
 * indivial allocations
 */
pub mod impl_2 {
    pub struct Tree<T> {
        pub value: T,
        pub left: Option<Box<Tree<T>>>,
        pub right: Option<Box<Tree<T>>>,
    }
}

/**
 * safe rust
 */
pub mod impl_3 {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    pub struct BinaryTree {
        r: Option<Rc<BTNode>>,
    }

    pub struct BTNode {
        value: i32, // or generic value T
        left: Option<Rc<RefCell<BTNode>>>,
        right: Option<Rc<RefCell<BTNode>>>,
        parent: Option<Weak<RefCell<BTNode>>>,
    }
}

// TODO DFS (pre-/in-/post-order), BFS
