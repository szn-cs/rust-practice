use std::cmp::Ord;

use specification::datastructure::binary_heap::minimal::BinaryHeap as Spec;

pub struct BinaryHeap<V, F>
where
    F: Fn(&V, &V) -> bool,
{
    comparator: F,
    vec: Vec<V>,
}

impl<V, F> BinaryHeap<V, F>
where
    V: Ord,
    F: Fn(&V, &V) -> bool,
{
    pub fn with_comparator(comparator: F) -> Self {
        Self {
            comparator,
            vec: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    fn left_child(index: usize) -> usize {
        index * 2 + 1
    }

    fn right_child(index: usize) -> usize {
        Self::left_child(index) + 1
    }
}

impl<V, F> Spec<V> for BinaryHeap<V, F>
where
    V: Ord,
    F: Fn(&V, &V) -> bool,
{
    fn push(&mut self, value: V) {
        self.vec.push(value);

        // Heapify up
        let mut current = self.len() - 1;
        while let Some(parent) = {
            if current == 0 {
                None
            } else {
                Some(Self::parent(current))
            }
        } {
            if (self.comparator)(&self.vec[current], &self.vec[parent]) {
                // let (left, right) = self.vec.split_at_mut(parent);
                // std::mem::swap(&mut left[0], &mut right[0]);
                self.vec.swap(current, parent);
            } else {
                break;
            }

            current = parent;
        }
    }

    fn pop(&mut self) -> Option<V> {
        if self.vec.len() == 0 {
            return None;
        }

        let r = self.vec.swap_remove(0);

        let len = self.vec.len();
        if len > 0 {
            // heapify down
            let mut current = 0;
            while Self::left_child(current) < len {
                let left_child = Self::left_child(current);
                let right_child = Self::right_child(current);

                let child = {
                    if (right_child < len) {
                        if (self.comparator)(&self.vec[left_child], &self.vec[right_child]) {
                            left_child
                        } else {
                            right_child
                        }
                    } else {
                        left_child
                    }
                };

                if (self.comparator)(&self.vec[child], &self.vec[current]) {
                    self.vec.swap(current, child);
                }

                current = child;
            }
        }

        Some(r)
    }
}
