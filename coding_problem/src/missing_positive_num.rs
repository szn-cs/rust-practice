/**
 * find first positive integer that is missing from the series [1..]
 */

// BF 1: for each number from [1..n] check if it exists in the array; O(n^2)T;
mod impl_1 {}

// BF 2: sort array then iterate over starting from value 1 in the array and compare to the iteration [1..n]; O(n log n)T; O(log n)S for in-place sorting
mod impl_2 {}

// Efficient: swap each value to it's index position, then iterate over array and pick the first position where index doesn't match value; O(n)T; O(1)S
mod impl_3 {
    pub fn missing_positive_integer(mut v: &mut [i32]) -> Option<i32> {
        let n: usize = v.len();

        // in-place swap to map 1..=n to index 0..n
        for i in 0..v.len() {
            let e = v[i];
            // care about only [1..] values
            if e > 0 && (e as usize) <= n {
                v.swap(i, e as usize - 1);
            }
        }

        {
            // alternative workaround exclusive mutable borrow more than once.
            let p0 = v.as_mut_ptr();
            for (i, &mut e) in v.iter_mut().enumerate() {
                // care about only [1..] values
                if e > 0 && (e as usize) <= n {
                    // v.swap(i, e as usize - 1); // issue: double borrow
                    // unsafe workaround:
                    unsafe {
                        let p1 = p0.offset(i as isize);
                        let p2 = p0.offset(e as isize - 1);
                        std::ptr::swap(p1, p2);
                    }
                }
            }
        }

        println!("{:?}", v);

        let mut r = None;
        for i in 0..v.len() {
            let e = v[i];
            let expected = i as i32 + 1;
            if expected != e {
                r = Some(expected);
                break;
            }
        }

        r
    }
}

// Efficient using hashset: store all values in a set, then for [1..n] check if numbers exist.
mod impl_4 {
    use std::collections::HashSet;

    pub fn missing_positive_integer(v: &[i32]) -> Option<i32> {
        let mut s = HashSet::new();

        for e in v {
            s.insert(e);
        }

        let mut r = None;

        for i in (1..=v.len() as i32) {
            if let None = s.get(&i) {
                r = Some(i);
                break;
            }
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_3() {
        use impl_3::missing_positive_integer;
        let mut v = vec![3, -1, 4, 1, 9];
        let r = missing_positive_integer(&mut v[..]);

        assert_eq!(r, Some(2));
    }

    #[test]
    fn case_4() {
        use impl_4::missing_positive_integer;
        let v = vec![3, -1, 4, 1, 9];
        let r = missing_positive_integer(&v[..]);

        assert_eq!(r, Some(2));
    }
}
