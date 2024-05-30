/**
 * Selection Sort
 * pick smallest from unsorted portion and place it in the sorted at largest elements.
 */
use specification::algorithm::sort::Sorter;

/**
 * loop to find smallest element
 */
pub mod impl_1 {
    use super::*;
    pub struct SelectionSort;

    impl Sorter for SelectionSort {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            for unsorted in 0..slice.len() {
                let mut min = unsorted;

                for i in (unsorted + 1)..slice.len() {
                    if compare(&slice[min], &slice[i]) {
                        // in case out-of-order then pick the earlier element in the sorted sequence.
                        min = i;
                    }
                }

                if unsorted != min {
                    slice.swap(unsorted, min);
                }
            }
        }
    }
}

/**
 * use iterators to find minimum
 */
pub mod impl_2 {
    use super::*;

    pub struct SelectionSort;

    impl Sorter for SelectionSort {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            for unsorted in 0..slice.len() {
                let (min, _) = slice[unsorted..]
                    .iter()
                    .enumerate()
                    .map(|(k, v)| (unsorted + k, v)) // adjust index to match index in entire slice
                    .min_by_key(|&(_, v)| v)
                    .expect("slice is empty");

                if min != unsorted {
                    slice.swap(unsorted, min);
                }
            }
        }
    }
}
