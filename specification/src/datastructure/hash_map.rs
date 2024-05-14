use std::iter::IntoIterator;

pub trait HashMap<K, V>: IntoIterator {
    // fields: len, bucket_list

    fn insert(&mut self, key: K, value: V) -> Option<V>; // if replace return the old value
    fn remove(&mut self, key: &K) -> Option<V>;

    fn resize(&mut self); // grow/extend buckets list to control sparsity

    fn search(&mut self, key: &K) -> Option<&V>; // either mutable or shared reference
    fn contains_key(&self, key: &K) -> bool;

    fn is_empty(&self) -> bool;
}

pub mod minimal {
    pub trait HashMap<K, V> {
        fn insert(&mut self, key: K, value: V) -> Option<V>;
        fn remove(&mut self, key: &K) -> Option<V>;
        fn extend(&mut self);
        fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    }
}
