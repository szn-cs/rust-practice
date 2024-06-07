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
                    if compare(&slice[0], &slice[1]) {
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
                if compare(element, &pivot) {
                    right.push(element.clone());
                } else {
                    left.push(element.clone());
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
    // use std::fmt::Display;
    pub struct QuickSort;

    impl QuickSort {
        pub fn sort<T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord, /* + Display */
            F: Fn(&T, &T) -> bool,
        {
            // base cases
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

            // choose appropriate pivot
            // let pick_pivot = |slice: &mut [T]| slice.len() / 2;
            let pick_pivot = |slice: &mut [T]| 0;
            let pivot = pick_pivot(slice); // index of pivot

            // partition to left/right portions
            slice.swap(0, pivot); // move pivot out of the way
            let (pivot, remain) = slice.split_first_mut().expect("slice is non-empty"); // remaining slice excluding pivot

            let split = {
                let mut left = 0; // bounds: [0, len]
                let mut right: usize = remain.len() - 1; // bounds: [overflow -1, len-1]

                while left <= right {
                    if remain[left] < *pivot {
                        left += 1;
                        // println!("left = {}", left);
                    } else if remain[right] > *pivot {
                        right = {
                            let right = right.checked_sub(1);
                            if let Some(right) = right {
                                right
                            } else {
                                break;
                            }
                        };
                        // println!("right = {}", right);
                    } else {
                        remain.swap(left, right);
                        left += 1;
                        right = {
                            let right = right.checked_sub(1);
                            if let Some(right) = right {
                                right
                            } else {
                                break;
                            }
                        };
                    };
                }

                left
            };

            // place pivot in its final adjusted position
            let first_right = split; // first element in right
            slice.swap(0, first_right);

            let (left, right) = slice.split_at_mut(split);
            let (_, right) = right.split_first_mut().expect("slice is non-empty"); // adjust to exclude pivot

            // recursively in-place sort portions (resulting in a sorted original slice)
            Self::sort(left, compare);
            Self::sort(right, compare);
        }
    }
}

/**
 * Use wrapping_sub
 * Same as impl_2 but with for loop for an additional bounding on number of iterations
 */
pub mod impl_3 {
    use super::*;
    // use std::fmt::Display;
    pub struct QuickSort;

    impl QuickSort {
        pub fn sort<T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord, /* + Display */
            F: Fn(&T, &T) -> bool,
        {
            // base cases
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

            // choose appropriate pivot
            // let pick_pivot = |slice: &mut [T]| slice.len() / 2;
            let pick_pivot = |slice: &mut [T]| 0;
            let pivot = pick_pivot(slice); // index of pivot

            // partition to left/right portions
            slice.swap(0, pivot); // move pivot out of the way
            let (pivot, remain) = slice.split_first_mut().expect("slice is non-empty"); // remaining slice excluding pivot

            let split = {
                let mut left = 0; // bounds: [0, len]
                let mut right: usize = remain.len() - 1; // bounds: [overflow -1, len-1]

                for _ in 0..remain.len() {
                    if left > right {
                        break;
                    }

                    if remain[left] < *pivot {
                        left += 1;
                        // println!("left = {}", left);
                    } else if remain[right] > *pivot {
                        right = right.wrapping_sub(1);
                        // println!("right = {}", right);
                    } else {
                        remain.swap(left, right);
                        left += 1;
                        right = right.wrapping_sub(1);
                    };
                }

                left
            };

            // place pivot in its final adjusted position
            let first_right = split; // first element in right
            slice.swap(0, first_right);

            let (left, right) = slice.split_at_mut(split);
            let (_, right) = right.split_first_mut().expect("slice is non-empty"); // adjust to exclude pivot

            // recursively in-place sort portions (resulting in a sorted original slice)
            Self::sort(left, compare);
            Self::sort(right, compare);
        }
    }
}

/**
 * Non-recursive iterative equivalent
 */
pub mod impl_4 {
    use super::*;
    use std::collections::VecDeque;

    pub struct QuickSort;

    impl QuickSort {
        pub fn sort<T, F>(slice: &mut [T], compare: &F)
        where
            T: Ord,
            F: Fn(&T, &T) -> bool,
        {
            let mut queue = VecDeque::new();

            queue.push_back(slice);

            while !queue.is_empty() {
                let slice = queue.pop_front().unwrap();

                match slice.len() {
                    0 | 1 => continue,
                    2 => {
                        if compare(&slice[0], &slice[1]) {
                            slice.swap(0, 1);
                        }
                        continue;
                    }
                    _ => {}
                }

                // perform partition by pivot and sort
                let (left, right) = {
                    // choose appropriate pivot
                    // let pick_pivot = |slice: &mut [T]| slice.len() / 2;
                    let pick_pivot = |slice: &mut [T]| 0;
                    let pivot = pick_pivot(slice); // index of pivot

                    // partition to left/right portions
                    slice.swap(0, pivot); // move pivot out of the way
                    let (pivot, remain) = slice.split_first_mut().expect("slice is non-empty"); // remaining slice excluding pivot

                    let split = {
                        let mut left = 0; // bounds: [0, len]
                        let mut right: usize = remain.len() - 1; // bounds: [overflow -1, len-1]

                        for _ in 0..remain.len() {
                            if left > right {
                                break;
                            }

                            if remain[left] < *pivot {
                                left += 1;
                                // println!("left = {}", left);
                            } else if remain[right] > *pivot {
                                right = right.wrapping_sub(1);
                                // println!("right = {}", right);
                            } else {
                                remain.swap(left, right);
                                left += 1;
                                right = right.wrapping_sub(1);
                            };
                        }

                        left
                    };

                    // place pivot in its final adjusted position
                    let first_right = split; // first element in right
                    slice.swap(0, first_right);

                    let (left, right) = slice.split_at_mut(split);
                    let (_, right) = right.split_first_mut().expect("slice is non-empty"); // adjust to exclude pivot

                    (left, right)
                };

                queue.push_back(left);
                queue.push_back(right);
            }
        }
    }
}
