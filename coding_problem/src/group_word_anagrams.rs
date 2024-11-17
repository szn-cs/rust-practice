/*
 *  given a vector of strings, group anagrams together
 */

mod impl_1 {
    use std::collections::BTreeMap;

    fn hash_key(s: &String) -> String {
        let mut encoded = s.chars().collect::<Vec<char>>();
        encoded.sort();
        let s = encoded.into_iter().collect::<String>();
        s
    }

    // use hashmap to group the words that are anagrams by a unique form of the anagram order/encoding i.e. using a hashing method.
    // O(n)T; O(n)S
    pub fn group_anagrams(v: Vec<String>) -> Vec<Vec<String>> {
        let mut h: BTreeMap<String, Vec<String>> = BTreeMap::new(); // using BTreeMap instead of HashMap just for the result to be returned in the same order.

        for e in v {
            let s = hash_key(&e);

            if let Some(v) = h.get_mut(&s) {
                v.push(e);
            } else {
                h.insert(s, vec![e]);
            }
        }

        let mut result = vec![];

        // convert to expected form
        for v in h.into_values() {
            result.push(v);
        }

        result
    }
}

mod impl_2 {
    use std::collections::BTreeMap;

    fn hash_key(s: &String) -> u32 {
        let mut bit_map = [0; 26]; // or could be configured to include all ASCII characters
        let char_start = 'a' as usize;

        for c in s.chars() {
            let c = c as usize - char_start;
            bit_map[c] = 1;
        }

        // convert into hashable
        let mut encoded: u32 = 0;
        let mut shift = 25;
        for b in bit_map {
            encoded |= b << shift;
            shift -= 1;
        }

        encoded
    }

    // use hashmap to group the words that are anagrams by a unique form of the anagram order/encoding i.e. using a hashing method.
    // O(n)T; O(n)S
    pub fn group_anagrams(v: Vec<String>) -> Vec<Vec<String>> {
        let mut h: BTreeMap<u32, Vec<String>> = BTreeMap::new(); // using BTreeMap instead of HashMap just for the result to be returned in the same order.

        for e in v {
            let s = hash_key(&e);

            if let Some(v) = h.get_mut(&s) {
                v.push(e);
            } else {
                h.insert(s, vec![e]);
            }
        }

        let mut result = vec![];

        // convert to expected form
        for v in h.into_values() {
            result.push(v);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_impl_1() {
        use impl_1::group_anagrams;

        let v: Vec<String> = vec!["abc".into(), "cde".into(), "cab".into(), "xyz".into()];

        let r = group_anagrams(v);

        assert_eq!(r, [vec!["abc", "cab"], vec!["cde"], vec!["xyz"]]);
    }

    #[test]
    fn case_impl_2() {
        use impl_2::group_anagrams;

        let v: Vec<String> = vec!["abc".into(), "cde".into(), "cab".into(), "xyz".into()];

        let r = group_anagrams(v);

        assert_eq!(r, [vec!["xyz"], vec!["cde"], vec!["abc", "cab"],]);
    }
}
