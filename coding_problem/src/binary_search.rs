use std::fmt::Debug;
/**
 * Req:
 * - slice must be sorted; violating this property is a logic error => resulting behavior is not specified.
 */
pub trait BinarySearch {
    fn search<T: Ord + Debug>(slice: &[T], target: &T) -> Option<usize>;
}

/**
 * Recursive using slices
 */
pub mod impl_1 {
    use super::*;
    use std::cmp::Ord;
    use std::cmp::Ordering;

    pub struct RecursiveBinarySearch;

    impl BinarySearch for RecursiveBinarySearch {
        fn search<T: Ord + Debug>(slice: &[T], target: &T) -> Option<usize> {
            match slice.len() {
                0 => return None,
                1 => return if &slice[0] == target { Some(0) } else { None },
                _ => {}
            }

            let m = slice.len() / 2;
            let (left, right) = slice.split_at(m);
            assert!(right.len() > 0);
            let (_, right) = right.split_first().unwrap();

            let cmp = target.cmp(&slice[m]);
            dbg!(cmp, m, target, &slice[m]);
            let mut index = match cmp {
                Ordering::Less => Self::search(left, target),
                Ordering::Equal => Some(m),
                Ordering::Greater => Self::search(right, target),
            };
            dbg!(index);

            if cmp == Ordering::Greater {
                index = index.map(|i| i + (m + 1)); /* adjust for subslice index */
            }

            index
        }
    }
}

/**
 * Recursive implementation using indecies.
 */
pub mod impl_2 {}

/**
 * Iterative
 */
pub mod impl_3 {}

#[cfg(test)]
mod tests_binary_search_recursive {
    use super::*;

    #[test]
    fn odd_length() {
        use impl_1::RecursiveBinarySearch;

        let v = vec![3, 5, 8, 9, 20, 50, 55, 60, 65, 66]; // n = 9

        let target = 5;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), Some(1));
        let target = 6;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), None);
        let target = 65;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), Some(8));
        let target = 66;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), Some(9));
    }

    #[test]
    fn empty_list() {
        use impl_1::RecursiveBinarySearch;

        let v = vec![];

        let target = 5;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), None);
        let target = -1;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), None);
    }

    #[test]
    fn negative_values() {
        use impl_1::RecursiveBinarySearch;

        let v = vec![-10, -1, 0, 1, 11];

        let target = 5;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), None);
        let target = -1;
        assert_eq!(RecursiveBinarySearch::search::<i32>(&v, &target), None);
    }

    // additional tests: single element, two elements, duplicate elements
}
