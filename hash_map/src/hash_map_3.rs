pub struct HashMap<K, V> {
    element_list: (K, V, bool), // (key, value, is_occupied)
    len: usize,
}
