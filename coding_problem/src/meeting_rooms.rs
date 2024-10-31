use std::cmp::{max, Eq, Ord, Ordering, PartialEq, Reverse};
/**
 * Calculate number of rooms required to fulfill meetings
 */
use std::collections::BinaryHeap;
use std::convert::From;

#[derive(Debug, Clone)]
pub struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn from_tuple_vec(v: &[(usize, usize)]) -> Vec<Interval> {
        v.into_iter().map(|&e| e.into()).collect::<Vec<Interval>>()
    }
}

impl From<(usize, usize)> for Interval {
    fn from(item: (usize, usize)) -> Self {
        Interval {
            start: item.0,
            end: item.1,
        }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        usize::cmp(&self.end, &other.end)
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.end.cmp(&other.end))
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end
    }
}

impl Eq for Interval {}

/**
 * for each interval (sorted by strat time) check if previous intervals are intersecting and update maximum interval value.
 * O(n log n)T; O(n)S;
 */
pub fn meeting_rooms(mut v: Vec<Interval>) -> usize {
    v.sort_by_key(|e| e.start);

    // min-heap over previous intervals
    let mut prev_heap: BinaryHeap<Reverse<Interval>> = BinaryHeap::from([Reverse(v[0].clone())]);
    let mut max_intersection = 1;

    for interval in v.iter().skip(1) {
        while let Some(Reverse(prev)) = prev_heap.peek() {
            if prev.end >= interval.start {
                break;
            }

            prev_heap.pop();
        }

        prev_heap.push(Reverse(interval.clone()));
        max_intersection = max(max_intersection, prev_heap.len());
    }

    max_intersection
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let v = Interval::from_tuple_vec(&[(1, 10), (11, 20), (21, 30)]);
        let r = meeting_rooms(v);
        assert_eq!(r, 1);
    }

    #[test]
    fn case_2() {
        let v = Interval::from_tuple_vec(&[(1, 10), (11, 16), (15, 20), (21, 30)]);
        let r = meeting_rooms(v);
        assert_eq!(r, 2);

        let v = Interval::from_tuple_vec(&[(21, 30), (11, 16), (15, 20), (1, 10)]);
        let r = meeting_rooms(v);
        assert_eq!(r, 2);
    }

    #[test]
    fn case_3() {
        let v = Interval::from_tuple_vec(&[(1, 50), (10, 21), (20, 31), (30, 60), (30, 40)]);
        let r = meeting_rooms(v);
        assert_eq!(r, 4);

        let v = Interval::from_tuple_vec(&[(10, 21), (30, 40), (30, 60), (20, 31), (1, 50)]);
        let r = meeting_rooms(v);
        assert_eq!(r, 4);
    }
}
