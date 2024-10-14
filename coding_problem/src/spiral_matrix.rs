mod impl_1 {
    use std::collections::HashSet;
    use std::string::ToString;

    #[derive(Clone)]
    enum Direction {
        Right,
        Bottom,
        Left,
        Top,
    }

    impl ToString for Direction {
        fn to_string(&self) -> String {
            match self {
                Self::Right => "right".to_owned(),
                Self::Left => "left".to_owned(),
                Self::Top => "top".to_owned(),
                Self::Bottom => "bottom".to_owned(),
            }
        }
    }

    fn next_direction(d: &Direction) -> Direction {
        use Direction::*;

        match &d {
            Right => Bottom,
            Bottom => Left,
            Left => Top,
            Top => Right,
        }
    }

    fn next_index(i: (usize, usize), d: &Direction) -> (i32, i32) {
        use Direction::*;
        let r = i.0 as i32;
        let c = i.1 as i32;

        match &d {
            Right => (r, c + 1),
            Bottom => (r + 1, c),
            Left => (r, c - 1),
            Top => (r - 1, c),
        }
    }

    fn is_valid_index(i: (i32, i32), n: usize, m: usize) -> bool {
        if !(0..n as i32).contains(&i.0) || !(0..m as i32).contains(&i.1) {
            false
        } else {
            true
        }
    }

    pub fn spiral_print(v: &Vec<Vec<i32>>) -> String {
        assert!(v.len() > 0 && v[0].len() > 0);

        let R: usize = v.len();
        let C: usize = v[0].len();
        let N: usize = v.len() * v[0].len();

        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut result = String::new();

        let mut i = (0, 0);
        let mut d = Direction::Right;
        for t in 1..=N {
            seen.insert(i);

            result.push_str(&format!("{}", v[i.0][i.1])[..]);
            result.push_str(if t != N { " " } else { "" });

            let mut next_i = next_index(i, &d);
            let is_valid = is_valid_index(next_i, R, C);
            let is_visited = seen.contains(&(next_i.0 as usize, next_i.1 as usize));

            if !is_valid || is_visited {
                // change direction
                d = next_direction(&d);
                next_i = next_index(i, &d); // must be valid after direction change
            }

            i = (next_i.0 as usize, next_i.1 as usize);
        }

        result
    }
}

// using iterators
mod impl_2 {
    use std::collections::HashSet;
    use std::iter::{IntoIterator, Iterator};

    pub struct Grid {
        matrix: Vec<Vec<i32>>,
        n: usize,
        m: usize,
    }

    impl Grid {
        pub fn spiral_print(self) -> String {
            self.into_iter()
                .fold(String::new(), |s, e| format!("{s} {e}"))
                .trim()
                .to_string()
        }

        pub fn new(v: Vec<Vec<i32>>) -> Grid {
            let N = v.len();
            assert!(N > 0);
            let M = v[0].len();

            Grid {
                matrix: v,
                n: N,
                m: M,
            }
        }

        fn next_index(&self, r: usize, c: usize, d: Direction) -> Option<(usize, usize)> {
            use Direction::*;

            let N = self.matrix.len();
            let M = self.matrix[0].len();

            match d {
                Right => {
                    if c + 1 < M {
                        Some((r, c + 1))
                    } else {
                        None
                    }
                }
                Bottom => {
                    if r + 1 < N {
                        Some((r + 1, c))
                    } else {
                        None
                    }
                }
                Left => {
                    if let Some(c) = c.checked_sub(1) {
                        Some((r, c))
                    } else {
                        None
                    }
                }
                Top => {
                    if let Some(r) = r.checked_sub(1) {
                        Some((r, c))
                    } else {
                        None
                    }
                }
            }
        }

        fn get(&self, r: usize, c: usize) -> i32 {
            self.matrix[r][c]
        }
    }

    #[derive(Clone)]
    enum Direction {
        Right,
        Bottom,
        Left,
        Top,
    }

    impl ToString for Direction {
        fn to_string(&self) -> String {
            use Direction::*;
            match self {
                Right => "right".to_string(),
                Left => "left".to_string(),
                Bottom => "bottom".to_string(),
                Top => "top".to_string(),
            }
        }
    }

    pub struct SpiralIterator {
        grid: Grid,
        current: (usize, usize),
        seen: HashSet<(usize, usize)>,
        direction: Direction,
        is_consumed: bool,
    }

    impl SpiralIterator {
        pub fn new(g: Grid) -> Self {
            SpiralIterator {
                grid: g,
                current: (0, 0),
                seen: HashSet::new(),
                direction: Direction::Right,
                is_consumed: false,
            }
        }

        fn next_direction(d: Direction) -> Direction {
            use Direction::*;

            match d {
                Right => Bottom,
                Bottom => Left,
                Left => Top,
                Top => Right,
            }
        }
    }

    impl Iterator for SpiralIterator {
        type Item = i32;

        // figure out next element & update state
        fn next(&mut self) -> Option<Self::Item> {
            if self.is_consumed {
                return None;
            }

            self.seen.insert(self.current);
            let value = self.grid.get(self.current.0, self.current.1);

            let mut next_index =
                self.grid
                    .next_index(self.current.0, self.current.1, self.direction.clone());

            if next_index.is_none()
                || self
                    .seen
                    .contains(&(next_index.unwrap().0, next_index.unwrap().1))
            {
                self.direction = Self::next_direction(self.direction.clone());
                next_index =
                    self.grid
                        .next_index(self.current.0, self.current.1, self.direction.clone());
            };

            match next_index {
                Some((r, c)) if !self.seen.contains(&(r, c)) => {
                    self.current = (r, c);
                }
                _ => {
                    self.is_consumed = true;
                }
            }

            Some(value)
        }
    }

    impl IntoIterator for Grid {
        type Item = i32;
        type IntoIter = SpiralIterator;

        fn into_iter(self) -> Self::IntoIter {
            SpiralIterator::new(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_1() {
        use impl_1::spiral_print;

        let v = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ];
        let s = spiral_print(&v);

        assert_eq!(s, "1 2 3 4 5 10 15 14 13 12 11 6 7 8 9");
    }

    #[test]
    fn impl_2() {
        use impl_2::Grid;

        let v = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ];
        let g = Grid::new(v);
        let s = g.spiral_print();

        assert_eq!(s, "1 2 3 4 5 10 15 14 13 12 11 6 7 8 9");
    }
}
