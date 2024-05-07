use datastructure::specification::hash_map::minimal::HashMap as Spec;
pub struct HashMap<K, V> {
    element_list: (K, V, bool), // (key, value, is_occupied)
    len: usize,
}

impl<K, V> Spec<K, V> for HashMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        unimplemented!();
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        unimplemented!();
    }

    fn extend(&mut self) {
        unimplemented!();
    }

    fn get_mut(&mut self, key: &key) -> Option<&mut V> {
        unimplemented!();
    }
}
