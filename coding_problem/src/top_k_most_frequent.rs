mod impl_1 {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};
    // hash map to count occurences and heap to get max k values
    pub fn top_k_most_frequent(v: &[i32], k: usize) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new(); // map of (value, counter)

        // count occurence
        for &e in v.iter() {
            *map.entry(e).or_insert(0) += 1;
        }

        // construct heap of max occurences
        let mut v = map
            .into_iter()
            .map(|e| (e.1, e.0))
            .collect::<Vec<(i32, i32)>>();
        let mut heap = BinaryHeap::from(v);

        // get k most frequent
        let mut result = vec![];
        for _ in 0..k {
            if let Some(value) = heap.pop() {
                result.push(value.1);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use impl_1::*;

    #[test]
    fn case_1() {
        let v = vec![2, 1, 5, 1, 1, 1, 2, 2, 3];
        let r = top_k_most_frequent(&v[..], 2);
        assert_eq!(r, [1, 2]);
    }
}
