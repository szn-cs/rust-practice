/**
 * BF O(n^3)T
 */
mod impl_1 {
    pub fn sum_3(v: &[i32]) -> Vec<(i32, i32, i32)> {
        assert!(v.len() > 3);

        let mut result: Vec<(i32, i32, i32)> = vec![];

        for i1 in 0..v.len() {
            for i2 in i1 + 1..v.len() {
                for i3 in i2 + 1..v.len() {
                    if v[i1] + v[i2] + v[i3] == 0 {
                        result.push((v[i1], v[i2], v[i3]));
                    }
                }
            }
        }

        result
    }
}

/**
 * reduce problem to 2_sum
 * O(n^2)T
 */
mod impl_2 {
    use std::collections::HashMap;

    // O(n) T; O(n) S;
    fn sum_2_map(v: &[i32], k: i32) -> Vec<(i32, i32)> {
        let mut result = vec![];
        let mut m: HashMap<i32, usize> = HashMap::new(); // complement value -> index

        for i in 0..v.len() {
            if let Some(&index) = m.get(&v[i]) {
                result.push((v[index as usize], v[i]));
            } else {
                m.insert(k - v[i], i);
            }
        }

        result
    }

    // assuming no repeating elements
    pub fn sum_3(v: &[i32]) -> Vec<(i32, i32, i32)> {
        if v.len() < 3 {
            return vec![];
        }

        let mut result: Vec<(i32, i32, i32)> = vec![];
        let mut v = v.to_vec();

        v.sort(); // O(n log n)T; O(n)S merge sort or O(log n) recursive quicksort;

        for k in 0..v.len() - 2 {
            let target = -1 * v[k] as i32;

            let r: Vec<(i32, i32, i32)> = sum_2_map(&v[k + 1..], target)
                .into_iter()
                .map(|(a, b)| (v[k], a, b))
                .collect();
            result.extend(r);
        }

        result
    }
}

/**
 * reduce problem to 2_sum with sum_2 pointer solution
 *
 */
mod impl_3 {
    // O(n) T; O(1)S ignoring result storing
    fn sum_2_pointer(v: &[i32], k: i32) -> Vec<(i32, i32)> {
        let mut result = vec![];

        let mut l = 0;
        let mut r = v.len() - 1;

        while l < r {
            let sum = v[l] + v[r];
            if sum < k {
                l += 1;
                continue;
            } else if sum > k {
                r -= 1;
                continue;
            }

            result.push((v[l], v[r]));
            l += 1;
            r -= 1;
        }

        result
    }

    // assuming no repeating elements
    pub fn sum_3(v: &[i32]) -> Vec<(i32, i32, i32)> {
        if v.len() < 3 {
            return vec![];
        }

        let mut result: Vec<(i32, i32, i32)> = vec![];
        let mut v = v.to_vec();

        v.sort(); // O(n log n)T; O(n)S merge sort or O(log n) recursive quicksort;

        for k in 0..v.len() - 2 {
            let target = -1 * v[k] as i32;

            let r: Vec<(i32, i32, i32)> = sum_2_pointer(&v[k + 1..], target)
                .into_iter()
                .map(|(a, b)| (v[k], a, b))
                .collect();
            result.extend(r);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_1() {
        use impl_1::sum_3;

        let v = vec![-1, 0, 1, 2, -4, -3];
        let r = sum_3(&v[..]);
        assert_eq!(r, [(-1, 0, 1), (1, 2, -3)]);
    }

    #[test]
    fn impl_2() {
        use impl_2::sum_3;

        let v = vec![-1, 0, 1, 2, -4, -3];
        let r = sum_3(&v[..]);
        assert_eq!(r, [(-3, 1, 2), (-1, 0, 1)]);
    }

    #[test]
    fn impl_3() {
        use impl_3::sum_3;

        let v = vec![-1, 0, 1, 2, -4, -3];
        let r = sum_3(&v[..]);
        assert_eq!(r, [(-3, 1, 2), (-1, 0, 1)]);
    }
}
