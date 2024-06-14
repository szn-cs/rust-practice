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
            // dbg!(cmp, m, target, &slice[m]);
            let mut index = match cmp {
                Ordering::Less => Self::search(left, target),
                Ordering::Equal => Some(m),
                Ordering::Greater => Self::search(right, target),
            };
            // dbg!(index);

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
pub mod impl_2 {
    use super::*;
    use std::{
        cmp::{Ord, Ordering},
        ops::Sub,
    };

    pub struct RecursiveBinarySearch;

    impl BinarySearch for RecursiveBinarySearch {
        /**
         * find index of value in list if exists
         *
         * Requirements:
         * - list should be sorted.
         * - left, right must be within bounds
         *
         * if assumptions are violated -> unexpected results / logic error.
         */
        fn search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
            // base/trivial cases
            if slice.len() == 0 {
                return None;
            }

            Self::search_helper(slice, target, 0, slice.len() - 1)
        }
    }

    impl RecursiveBinarySearch {
        fn search_helper<T: Ord>(
            slice: &[T],
            target: &T,
            left: usize,
            right: usize,
        ) -> Option<usize> {
            if left < right && left > slice.len() && right < slice.len() {
                return None;
            }

            let middle = ((right + left) as f64 / 2f64).floor() as usize; // get arithmetic mean

            match target.cmp(&slice[middle]) {
                Ordering::Equal => Some(middle),
                Ordering::Greater => Self::search_helper(slice, target, middle + 1, right),
                Ordering::Less => {
                    if let Some(right) = middle.checked_sub(1) {
                        Self::search_helper(slice, target, left, right)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

/**
 * Iterative
 */
pub mod impl_3 {}

#[cfg(test)]
mod tests_binary_search {
    use super::*;

    mod test_impl_1 {
        use super::super::*;
        use impl_1::RecursiveBinarySearch as BS;

        #[test]
        fn odd_length() {
            let v = vec![3, 5, 8, 9, 20, 50, 55, 60, 65, 66]; // n = 9

            let target = 5;
            assert_eq!(BS::search::<i32>(&v, &target), Some(1));
            let target = 6;
            assert_eq!(BS::search::<i32>(&v, &target), None);
            let target = 65;
            assert_eq!(BS::search::<i32>(&v, &target), Some(8));
            let target = 66;
            assert_eq!(BS::search::<i32>(&v, &target), Some(9));
        }

        #[test]
        fn empty_list() {
            let v = vec![];

            let target = 5;
            assert_eq!(BS::search::<i32>(&v, &target), None);
            let target = -1;
            assert_eq!(BS::search::<i32>(&v, &target), None);
        }

        #[test]
        fn negative_values() {
            let v = vec![-10, -1, 0, 1, 11];

            let target = 5;
            assert_eq!(BS::search::<i32>(&v, &target), None);
            let target = -1;
            assert_eq!(BS::search::<i32>(&v, &target), Some(1));
        }

        // additional tests: single element, two elements, duplicate elements
    }

    mod test_impl_2 {
        use super::super::*;
        use impl_2::RecursiveBinarySearch as BS;

        #[test]
        fn odd_length() {
            let v = vec![3, 5, 8, 9, 20, 50, 55, 60, 65, 66]; // n = 9

            let target = 5;
            assert_eq!(BS::search::<i32>(&v, &target), Some(1));
            let target = 6;
            assert_eq!(BS::search::<i32>(&v, &target), None);
            let target = 65;
            assert_eq!(BS::search::<i32>(&v, &target), Some(8));
            let target = 66;
            assert_eq!(BS::search::<i32>(&v, &target), Some(9));
        }

        #[test]
        fn empty_list() {
            let v = vec![];

            let target = 5;
            assert_eq!(BS::search::<i32>(&v, &target), None);
            let target = -1;
            assert_eq!(BS::search::<i32>(&v, &target), None);
        }

        #[test]
        fn negative_values() {
            let v = vec![-10, -1, 0, 1, 11];

            let target = 5;
            assert_eq!(BS::search::<i32>(&v, &target), None);
            let target = -1;
            assert_eq!(BS::search::<i32>(&v, &target), Some(1));
        }

        // additional tests: single element, two elements, duplicate elements
    }
}
