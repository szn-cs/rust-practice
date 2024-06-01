use specification::algorithm::sort::Sorter;

/**
 * Inefficient space complexity, allocation itermediate vectors
 * vectors to split the slice on each recursive iteration.
 */
pub mod impl_1 {
    use super::*;
    use std::fmt::Display;

    pub struct QuickSort;

    impl QuickSort {
        pub fn sort<'a, T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord + Default + Clone + Display,
            F: Fn(&T, &T) -> bool,
        {
            // consider base cases
            match slice.len() {
                0 | 1 => return,
                2 => {
                    if slice[0] > slice[1] {
                        slice.swap(0, 1);
                    }
                    return;
                }
                _ => {}
            }

            let pick_pivot_fn = || 0;
            let p = pick_pivot_fn();
            let pivot = slice[p].clone();

            // sort left/right relative to pivot
            let mut left = vec![];
            let mut right = vec![];

            for element in slice
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != p)
                .map(|(_, value)| value)
            {
                if *element <= pivot {
                    left.push(element.clone());
                } else {
                    right.push(element.clone());
                }
            }

            assert_eq!(left.len() + 1 + right.len(), slice.len()); // sanity check

            // sort parts
            Self::sort(&mut left[..], compare);
            Self::sort(&mut right[..], compare);

            // merge sorted parts
            slice[..left.len()].clone_from_slice(&left[..]);
            slice[left.len()] = pivot;
            slice[(left.len() + 1)..].clone_from_slice(&right[..]);
        }
    }
}

/**
 * In-slice swapping elements
 * split into sub-slices and swap in-place
 */
pub mod impl_2 {
    use super::*;
    pub struct QuickSort;

    impl Sorter for QuickSort {
        fn sort<T, F>(slice: &mut [T], compare: F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
        }
    }
}
