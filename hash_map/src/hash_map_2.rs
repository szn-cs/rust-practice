pub struct HashMap<K, V> {
    bucket_list: Vec<Vec<(K, V)>>,
    len: usize,
}
