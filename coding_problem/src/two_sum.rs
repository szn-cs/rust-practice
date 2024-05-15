// https://leetcode.com/problems/two-sum/
/**
 * Find pair that satisfies a predicate
 *
 * constraits: list has no duplicates; There is only a single solution;
 */

/**
 * Brute force O(n^2)
 */
mod impl_1 {
    pub fn satisfies_predicate<T, F>(list: &[T], predicate: F) -> Option<(usize, usize)>
    where
        F: Fn(&T, &T) -> bool,
    {
        let mut pair: Option<(usize, usize)> = None;

        'outer: for i in 0..list.len() {
            for j in 0..list.len() {
                if i == j {
                    continue;
                }

                if (predicate)(&list[i], &list[j]) {
                    pair = Some((i, j));
                    break 'outer;
                }
            }
        }

        pair
    }
}

/**
 * optimization: skip repeated work
 */
mod impl_2 {
    pub fn satisfies_predicate<T, F>(list: &[T], predicate: F) -> Option<(usize, usize)>
    where
        F: Fn(&T, &T) -> bool,
    {
        let mut pair: Option<(usize, usize)> = None;

        'outer: for i in 0..list.len() {
            for j in (i + 1)..list.len() {
                if (predicate)(&list[i], &list[j]) {
                    pair = Some((i, j));
                    break 'outer;
                }
            }
        }

        pair
    }
}

/**
 * Sort then probe for complement using binary search
 */
mod impl_3 {
    pub fn two_sum(list: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut r: Option<(usize, usize)> = None;

        // sort elements
        let mut list = {
            let mut l = Vec::with_capacity(list.len());
            l.extend_from_slice(list);
            let mut l: Vec<(usize, i32)> = l.iter().enumerate().map(|(i, &v)| (i, v)).collect();
            (&mut l[..]).sort_by(|&(_, a), (_, b)| a.cmp(b));
            l
        };

        // find pair
        for &(i, e) in list.iter() {
            let seek = target - e; // complement of comparison equation

            // check if value exist in list using binary search
            if let Ok(sorted_index) = (&list[..]).binary_search_by(|&(_, v)| v.cmp(&seek)) {
                let (j, _) = list[sorted_index]; // extract index in the original list
                r = Some((i, j));
                break;
            }
        }

        r
    }
}

/**
 * Probe for complement using hash map
 */
mod impl_4 {
    // TODO!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_1() {
        use super::impl_1::satisfies_predicate;

        {
            let l = vec![2, 7, 11, 15];
            let target = 9;
            let result = satisfies_predicate(&l[..], |&a, &b| a + b == target);
            assert_eq!(result, Some((0, 1)));
        }
        {
            let l = vec![3, 2, 4];
            let target = 6;
            let result = satisfies_predicate(&l[..], |&a, &b| a + b == target);
            assert_eq!(result, Some((1, 2)));
        }
        {
            let l = vec![3, 3];
            let target = 6;
            let result = satisfies_predicate(&l[..], |&a, &b| a + b == target);
            assert_eq!(result, Some((0, 1)));
        }
    }

    #[test]
    fn test_impl_2() {
        use super::impl_2::satisfies_predicate;

        {
            let l = vec![2, 7, 11, 15];
            let target = 9;
            let result = satisfies_predicate(&l[..], |&a, &b| a + b == target);
            assert_eq!(result, Some((0, 1)));
        }
        {
            let l = vec![3, 2, 4];
            let target = 6;
            let result = satisfies_predicate(&l[..], |&a, &b| a + b == target);
            assert_eq!(result, Some((1, 2)));
        }
        {
            let l = vec![3, 3];
            let target = 6;
            let result = satisfies_predicate(&l[..], |&a, &b| a + b == target);
            assert_eq!(result, Some((0, 1)));
        }
    }

    #[test]
    fn test_impl_3() {
        use super::impl_3::two_sum;

        {
            let l = vec![11, 2, 15, 7];
            let result = two_sum(&l[..], 9);
            assert_eq!(result, Some((1, 3)));
        }
        {
            let l = vec![3, 2, 4];
            let result = two_sum(&l[..], 6);
            assert_eq!(result, Some((1, 2)));
        }
        {
            let l = vec![3, 3];
            let result = two_sum(&l[..], 6);
            assert_eq!(result, Some((0, 1)));
        }
    }
}
