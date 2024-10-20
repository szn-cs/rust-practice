/**
 * Insertion sort
 * divide slice into [sorted | unsorted] and move each element to correct position.
 */
use specification::algorithm::sort::Sorter;
use std::cmp::Ord;
use std::ops::Fn;

/**
 * swap each element to correct position; moving elements from unsorted to sorted portion.
 */
pub mod impl_1 {
    use super::*;
    pub struct InsertionSort;

    impl Sorter for InsertionSort {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            // [ sorted = 0 | unsorted = 1..len ]
            for mut i in 1..slice.len() {
                while i > 0 && compare(&slice[i - 1], &slice[i]) {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            }
        }
    }
}

/**
 * Reverse implementation
 */
pub mod impl_2 {
    use super::*;
    pub struct InsertionSort;

    impl Sorter for InsertionSort {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            if slice.len() == 0 {
                return;
            }

            let last = slice.len() - 1;
            for mut i in (0..last).rev() {
                while i < last && compare(&slice[i], &slice[i + 1]) {
                    slice.swap(i, i + 1);
                    i += 1;
                }
            }
        }
    }
}

/**
 * modified version with binary search; utilize slice binary search method to determine exact insertion position
 */
pub mod impl_3 {
    use super::*;

    pub struct InsertionSort;

    impl Sorter for InsertionSort {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            for unsorted in 1..slice.len() {
                let target = slice[..unsorted]
                    .binary_search(&slice[unsorted])
                    .unwrap_or_else(|i| i); // position to insert the element
                slice[target..=unsorted].rotate_right(1); // shift and last element wraps
            }
        }
    }
}
