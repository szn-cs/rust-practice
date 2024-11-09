/**
 * get all elements at a certain depth level of a binary tree
 */
use std::ptr::NonNull;
use std::string::ToString;

type Link = Option<NonNull<Node>>;

#[derive(Clone)]
struct Node {
    value: i32,
    left: Link,
    right: Link,
}

impl Node {
    pub fn new(value: i32) -> Option<NonNull<Self>> {
        NonNull::new(Box::into_raw(Box::new(Node {
            value,
            left: None,
            right: None,
        })))
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut current = NonNull::new(&mut (*self).clone());

        let mut stack = vec![];
        stack.push(current);
        while let Some(current) = stack.pop() {
            if let Some(current_ptr) = current {
                let current_ref = unsafe { current_ptr.as_ref() };
                result = format!("{result} {}", current_ref.value);
                stack.push(current_ref.right);
                stack.push(current_ref.left);
            }
        }

        result.trim().to_owned()
    }
}

/**
 * DFS till target level; O(n)T; O(n)S for non-balanced trees.
 */
pub fn values_at_level(node: NonNull<Node>, depth: usize) -> Vec<i32> {
    let node_ref = unsafe { node.as_ref() };

    if depth == 1 {
        return vec![node_ref.value];
    }

    let (mut l, mut r) = (
        if let Some(left) = node_ref.left {
            values_at_level(left, depth - 1)
        } else {
            vec![]
        },
        if let Some(right) = node_ref.right {
            values_at_level(right, depth - 1)
        } else {
            vec![]
        },
    );

    let mut result = l;
    result.append(&mut r);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree;

        unsafe {
            tree = Node::new(1);
            let mut tree = tree.unwrap();
            tree.as_mut().left = Node::new(2);
            tree.as_mut().right = Node::new(3);
            {
                let mut tree = tree.as_mut().left.unwrap();
                tree.as_mut().left = Node::new(4);
                tree.as_mut().right = Node::new(5);
            }
            {
                let mut tree = tree.as_mut().right.unwrap();
                tree.as_mut().right = Node::new(7);
            }
        };

        // let s = unsafe { tree.unwrap().as_ref().to_string() };
        // println!("{s}");

        let result = values_at_level(tree.unwrap(), 3);

        assert_eq!(result, [4, 5, 7]);
    }
}
