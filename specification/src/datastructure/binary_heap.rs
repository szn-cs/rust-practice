use std::cmp::Ord;

pub trait BinaryHeap<V>
where
    V: Ord,
{
    fn push(&mut self, value: V); // insertion/add
    fn pop(&mut self) -> Option<V>; // removal/deletion
    fn peak(&self) -> Option<&V>;
    // fn heap_sort()
}

pub mod minimal {
    pub trait BinaryHeap<V>
    where
        V: Ord,
    {
        // fields: len, vector list

        fn push(&mut self, value: V); // insertion
        fn pop(&mut self) -> Option<V>; // removal
    }
}
