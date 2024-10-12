/**
 * sort then pick k elements
 * O(n log n)T; O(log n)S with in-place sorting (e.g. quicksort)
 */
mod impl_1 {
    use std::cmp::Reverse;

    pub fn find_kth_max(v: &mut [i32], k: usize) -> i32 {
        v.sort_by_key(|&e| Reverse(e));
        v[k - 1]
    }
}

/**
 *  Heap DS
 *  O(n)T; O(n)S;
 */
mod impl_2 {
    use std::collections::BinaryHeap;

    pub fn find_kth_max(v: Vec<i32>, k: usize) -> Option<i32> {
        let mut h: BinaryHeap<i32> = BinaryHeap::from(v); // O(n)

        let mut element = None;
        for _ in 1..=k {
            element = h.pop();
        }

        element
    }
}

/**
 * Quick select up until proper index
 * O(log n)S; O(log n)T average
 */
mod impl_3 {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::ops::RangeBounds;

    fn pick_pivot(s: usize, e: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(s..=e)
    }

    fn quick_select(
        v: &mut [i32],
        l: usize,
        r: usize,
        target_index: usize, /* target index to select */
    ) -> Option<i32> {
        // base case
        match r.saturating_sub(l) {
            1 => {
                // two elements left
                let mut r = r;
                let mut l = l;
                if v[l] > v[r] {
                    v.swap(l, r);
                }

                l += 1;
                r -= 1;

                return if l == target_index {
                    Some(v[l])
                } else if r == target_index {
                    Some(v[r])
                } else {
                    None
                };
            }
            0 => return if l == target_index { Some(v[l]) } else { None },
            _ => {}
        }

        let mut pivot_index = pick_pivot(l, r);

        // swap pivot
        let pivot = v[pivot_index];
        v.swap(l, pivot_index);
        pivot_index = l;

        {
            // partition
            let mut l = l + 1; // skip pivot
            let mut r = r;
            while l <= r {
                if v[l] < pivot {
                    l += 1;
                    continue;
                }

                if v[r] > pivot {
                    r -= 1;
                    continue;
                }

                // both l/r elements are out of order
                v.swap(l, r);
                r -= 1; // subtraction range = [0..]
                l += 1;
            }

            // place pivot into position
            v.swap(pivot_index, r); // last of small portion
            pivot_index = r;
        }

        match target_index.cmp(&pivot_index) {
            Ordering::Equal => Some(v[pivot_index]),
            Ordering::Less => quick_select(v, l, pivot_index.saturating_sub(1), target_index),
            Ordering::Greater => quick_select(v, pivot_index + 1, r, target_index),
        }
    }

    pub fn find_kth_max(v: &mut [i32], k: usize) -> Option<i32> {
        quick_select(v, 0, v.len() - 1, v.len() - k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_1() {
        use impl_1::find_kth_max;

        let mut v = vec![3, 7, 1, 4, 2, 9, 8, 6];
        let element = find_kth_max(&mut v[..], 4);

        assert_eq!(element, 6);
    }

    #[test]
    fn impl_2() {
        use impl_2::find_kth_max;

        let mut v = vec![3, 7, 1, 4, 2, 9, 8, 6];
        let element = find_kth_max(v, 4);

        assert_eq!(element, Some(6));
    }

    #[test]
    fn impl_3() {
        use impl_3::find_kth_max;

        let mut v = vec![3, 7, 1, 4, 2, 9, 8, 6];
        let element = find_kth_max(&mut v[..], 4);

        assert_eq!(element, Some(6));
    }
}
