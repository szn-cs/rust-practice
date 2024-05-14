pub trait Sorter {
    fn sort<T, F>(slice: &mut [T], compare: F)
    where
        T: Ord,
        F: Fn(&T, &T) -> bool; // returns true if out of order
}

/**
 * Sort a slice using a provided Sorter implementation
 */
pub fn sort<T, S, F>(slice: &mut [T], compare: F)
where
    T: Ord,
    S: Sorter,
    F: Fn(&T, &T) -> bool,
{
    S::sort(slice, compare);
}
