use specification::algorithm::sort::Sorter;
use std::fmt::{Debug, Display};

/**
 * Copy elements
 * divide & merge elements into new allocations
 *
 * Not efficient on the space complexity side.
 */
pub mod impl_1 {
    use super::*;

    pub struct MergeSort;

    impl MergeSort {
        pub fn sort<T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord + Clone, /*+ Default + Display + Debug*/
            F: Fn(&T, &T) -> bool,
        {
            // base cases & trivial cases
            match slice.len() {
                0 | 1 => return,
                2 => {
                    if compare(&slice[0], &slice[1]) {
                        slice.swap(0, 1);
                    }
                    return;
                }
                _ => {}
            }

            // [1] partition & sort left/right
            let n = slice.len();
            let m = n / 2; // ex. 4 -> 2 [0,1,2,3]
            let (left, right) = slice.split_at_mut(m);
            // println!("left len: {}", left.len());
            Self::sort(left, compare);
            // println!("right len: {}", right.len());
            Self::sort(right, compare);
            // println!("done partition sort");

            // [2] merge two sorted partitions
            let mut sorted = {
                let mut s = Vec::with_capacity(n);
                let mut l = 0;
                let mut r = 0;

                loop {
                    // println!("indecies ({}, {})", l, r);
                    // dbg!(&s);
                    if l < left.len() && r < right.len() {
                        if compare(&left[l], &right[r]) {
                            s.push(right[r].clone());
                            // println!("right {}", right[r]);
                            r += 1;
                        } else {
                            s.push(left[l].clone());
                            // println!("left {}", left[l]);
                            l += 1;
                        }
                    } else if l < left.len() {
                        s.extend_from_slice(&left[l..]);
                        // println!("left {}", left[l]);
                        break;
                    } else if r < right.len() {
                        s.extend_from_slice(&right[r..]);
                        // println!("right {}", right[r]);
                        break;
                    } else {
                        break; // exhausted both slices' elements
                    }
                }

                s
            };

            // memory sorted elements copy into original slice (hackish solution instead of relying on helper function)
            // dbg!(&slice);
            // dbg!(&sorted);
            assert_eq!(slice.len(), sorted.len()); // sanity check
            slice.clone_from_slice(&sorted[..]);
        }
    }
}

/**
 * idiomatic Rust using iterators + using new allocations
 */
pub mod impl_2 {
    use super::*;
    use std::iter::zip;

    pub struct MergeSort;

    impl MergeSort {
        pub fn sort<T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord + Clone,
            F: Fn(&T, &T) -> bool,
        {
            // base/trivial cases
            match slice.len() {
                0 | 1 => return,
                2 => {
                    if compare(&slice[0], &slice[1]) {
                        slice.swap(0, 1);
                    }
                    return;
                }
                _ => {}
            }

            assert!(slice.len() > 1); // sanity check

            // [1] partition & sort left/right
            let n = slice.len();
            let m = slice.len() / 2; // e.g. [0,1,2,3] -> 2; [0,1,2,3,4] -> 2;
            let (left, right) = slice.split_at_mut(m); // ( [..m), [m..len-1] )
            Self::sort(left, compare);
            Self::sort(right, compare);

            // [2] Merge left/right partitions
            let sorted = {
                let mut s = Vec::with_capacity(n);
                let mut left = left.iter().peekable();
                let mut right = right.iter().peekable();

                while let (Some(l), Some(r)) = (left.peek(), right.peek()) {
                    s.push(if compare(l, r) {
                        right.next().unwrap().clone()
                    } else {
                        left.next().unwrap().clone()
                    });
                }

                s.extend(left.cloned());
                s.extend(right.cloned());

                s
            };

            // reflect sorted elements on original list
            assert_eq!(sorted.len(), slice.len());
            slice.clone_from_slice(&sorted[..]);
        }
    }
}

/**
 * Swap elements in-place
 */
pub mod impl_3 {
    pub struct MergeSort;

    impl MergeSort {
        pub fn sort<T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            // base cases & trivial cases
            match slice.len() {
                0 | 1 => return,
                2 => {
                    if compare(&slice[0], &slice[1]) {
                        slice.swap(0, 1);
                    }
                    return;
                }
                _ => {}
            }

            // [1] partition & sort
            let n = slice.len();
            let m = n / 2; // ex. 4 -> 2 [0,1,2,3]
            let (left, right) = slice.split_at_mut(m);

            Self::sort(left, compare);
            Self::sort(right, compare);

            // [2] merge in-place
            let (mut i, mut j) = (0, m);
            while i < j && j < n {
                if compare(&slice[i], &slice[j]) {
                    slice[i..=j].rotate_right(1);
                    j += 1;
                }
                i += 1;
            }
        }
    }
}

/**
 * Non-recursive equivalent iterative solution using a stack
 */
pub mod impl_4 {
    use super::*;
    use std::vec::Vec;

    pub struct MergeSort;

    pub fn sort<T, F>(slice: &mut [T], compare: &F)
    where
        T: Ord,
        F: Fn(&T, &T) -> bool,
    {
        /*
        let n = slice.len();

        if n < 2 {
            return;
        }

        // init stack
        let mut stack = {
            let mut v = Vec::new();
            v.push(slice);
            v
        };

        while !stack.is_empty() {
            let partition = stack.pop().unwrap();
            let n = partition.len();
            let m = partition.len() / 2; // [0, mid) and [mid, len-1]

            let (left, right) = partition.split_at_mut(m);

            stack.push(left);
            stack.push(right);

            // base/trivial cases
            match n {
                0 | 1 => continue,
                2 => {
                    if compare(&partition[0], &partition[1]) {
                        partition.swap(0, 1);
                        continue;
                    }
                }
                _ => {}
            }
        }

        // [2] merge left/right partitions
         */
    }
}

/**
 * Non-iterative using nested for loop (rephrasing the algorithm to implement a bottom-up approach)
 */
pub mod impl_5 {
    use super::*;
    use std::cmp::min;
    use std::fmt::Debug;
    use std::iter;

    pub struct MergeSort;

    impl MergeSort {
        fn merge<T, F>(slice: &mut [T], compare: &F, left: usize, right: usize)
        where
            T: Ord + Debug,
            F: Fn(&T, &T) -> bool,
        {
            let (mut i, mut j) = (left, right); // left/right partitions

            while i < j && j < slice.len() {
                if compare(&slice[i], &slice[j]) {
                    slice[i..=j].rotate_right(1);
                    j += 1;
                };
                i += 1;
            }
        }

        pub fn sort<T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord + Debug,
            F: Fn(&T, &T) -> bool,
        {
            let n = slice.len();

            let window = {
                let mut counter = 1;

                iter::from_fn(move || {
                    let p = 2usize.pow(counter);
                    counter += 1;
                    Some(p)
                }) // e.g. 2, 4, 8, 16, ..
                .take_while(|&p| p < n * 2) // will generate partitions (powers of 2) that are either engulfed within n, or the smallest partition that engulfs n itself. i.e. if number lands between power 2 partitions, then choose the higher one.
            };

            let group = |w: usize| {
                let mut l = 0;
                let it = iter::from_fn(move || {
                    if  l >= n /*out of range */ 
                        || 
                        l == n - 1 /* last element */ {
                        return None;
                    }

                    let h = min(l + w - 1, n - 1);
                    let m = w / 2 + l;

                    // skip remaining elements if don't fit window partition
                    if m >= n {
                        return None; 
                    }

                    let tuple = (l, m, h);

                    l += w;

                    Some(tuple)
                });

                it
            };

            for w in window {
                for (l, m, h) in group(w).into_iter() {
                    println!("Window size: {} partition: {} {} {}", w, l, m, h);
                    let slice = &mut slice[l..=h]; // window constrained portion
                    dbg!(&slice);
                    Self::merge(slice, compare, 0, m - l);
                    dbg!(&slice);
                }
            }
        }
    }
}
