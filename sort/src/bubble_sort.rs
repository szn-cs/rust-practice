/**
 * Bubble sort
 * each iteration results with larger items placed in the correct position on the right (bubbles largest element).
 */
use specification::algorithm::sort::Sorter;

/**
 * O(n^2) nested loop implementation
 */
pub mod impl_1 {
    use super::*;

    pub struct BubbleSorter;

    impl Sorter for BubbleSorter {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            for _ in 0..slice.len() {
                // sweep array swapping to position an element in correct position (ends up being last element)
                for i in 0..(slice.len() - 1) {
                    if (compare)(&slice[i], &slice[i + 1]) {
                        slice.swap(i, i + 1);
                    }
                }
            }
        }
    }
}

/**
 * Minor optimization; prevent unnecessary iterations if list already sorted.
 *
 * for loop flavor
 */
pub mod impl_2 {
    use super::*;

    pub struct BubbleSorter;

    impl Sorter for BubbleSorter {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            let mut swapped = true;
            for _ in 0..slice.len() {
                if !swapped {
                    break; // break early if last sweep happened on sorted list
                }

                swapped = false;
                // sweep array swapping to position an element in correct position (ends up being last element)
                for i in 0..(slice.len() - 1) {
                    if (compare)(&slice[i], &slice[i + 1]) {
                        slice.swap(i, i + 1);
                        swapped = true;
                    }
                }
            }
        }
    }
}

/**
 * Minor optimization; prevent unnecessary iterations if list already sorted.
 * (minor optimization to quit early if already sorted)
 *
 * while flavor
 */
pub mod impl_3 {
    use super::*;

    pub struct BubbleSorter;

    impl Sorter for BubbleSorter {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            let mut swapped = true;
            // repeat until entire list is sorted
            while swapped {
                swapped = false;
                // sweep array swapping to position an element in correct position (ends up being last element)
                for i in 0..(slice.len() - 1) {
                    if (compare)(&slice[i], &slice[i + 1]) {
                        slice.swap(i, i + 1);
                        swapped = true;
                    }
                }
            }
        }
    }
}

/**
 * Reverse logic current & next to current & prev
 */
pub mod impl_4 {
    use super::*;

    pub struct BubbleSorter;

    impl Sorter for BubbleSorter {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            let mut swapped = true;
            while swapped {
                swapped = false;
                for i in 1..slice.len() {
                    if (compare)(&slice[i - 1], &slice[i]) {
                        slice.swap(i - 1, i);
                        swapped = true;
                    }
                }
            }
        }
    }
}
