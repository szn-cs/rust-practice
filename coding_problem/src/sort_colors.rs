// given a 3 colors array, sort the colors

/**
 * count colors and expand
 * O(n)T; O(n)S; otherwise in-place modification can be used
 **/
mod impl_1 {
    pub fn sort_colors(v: Vec<usize>) -> Vec<usize> {
        let mut result: Vec<usize> = v.clone();
        let mut count: [usize; 3] = [0; 3];

        // count occurences
        for e in v {
            count[e] += 1;
        }

        // expand colors
        let start = 0;
        for i in 0..count[0] {
            let i = start + i;
            result[i] = 0;
        }

        let start = count[0];
        for i in 0..count[1] {
            let i = start + i;
            result[i] = 1;
        }

        let start = count[0] + count[1];
        for i in 0..count[2] {
            let i = start + i;
            result[i] = 2;
        }

        result
    }
}

mod impl_2 {
    use std::ops::Range;

    pub fn sort_colors(v: &mut Vec<usize>) {
        let mut count: [usize; 3] = [0; 3];

        // count occurence
        for &e in v.iter() {
            count[e] += 1;
        }

        // expand occurences
        let mut range_list = [
            if count[0] > 0 {
                Some(Range {
                    start: 0,
                    end: count[0],
                })
            } else {
                None
            },
            if count[1] > 0 {
                Some(Range {
                    start: count[0],
                    end: count[0] + count[1],
                })
            } else {
                None
            },
            if count[2] > 0 {
                Some(Range {
                    start: count[0] + count[1],
                    end: count[0] + count[1] + count[2],
                })
            } else {
                None
            },
        ];

        let mut color = 0;
        for i in 0..v.len() {
            while let Some(r) = &range_list[color] {
                if !r.contains(&i) {
                    color += 1;
                }

                break;
            }

            v[i] = color;
        }
    }
}

/**
 * swap colors till partitioned into left, middle, right;
 *
 * O(n)T; O(1)S;
 */
mod impl_3 {
    pub fn sort_colors(v: &mut Vec<usize>) {
        let (mut l, mut r) = (0, v.len() - 1);
        let mut current = 0;

        while current <= r {
            if v[current] == 0 {
                v.swap(current, l);
                l += 1;
                current += 1;
            } else if v[current] == 2 {
                v.swap(current, r);
                let checked_sub = r.checked_sub(1);
                if checked_sub.is_none() {
                    break;
                };
                r = checked_sub.unwrap();
            } else {
                current += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        use impl_1::sort_colors;

        let v = [1, 0, 1, 2, 0, 2, 0, 1, 0, 2];
        let v = v.to_vec();
        let r = sort_colors(v);
        assert_eq!(r, [0, 0, 0, 0, 1, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn case_2() {
        use impl_2::sort_colors;

        let mut v = [1, 0, 1, 2, 0, 2, 0, 1, 0, 2];
        let mut v = v.to_vec();
        sort_colors(&mut v);
        assert_eq!(v, [0, 0, 0, 0, 1, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn case_3() {
        use impl_3::sort_colors;

        let mut v = [1, 0, 1, 2, 0, 2, 0, 1, 0, 2];
        let mut v = v.to_vec();
        sort_colors(&mut v);
        assert_eq!(v, [0, 0, 0, 0, 1, 1, 1, 2, 2, 2]);
    }
}
