trait Fibunacci {
    fn generate_fib(p: u64) -> u64;
}

/**
 * Recursive exhaustive
 */
mod impl_1 {
    use super::*;
    pub struct FibGenerator;

    impl Fibunacci for FibGenerator {
        fn generate_fib(p: u64) -> u64 {
            match p {
                0 | 1 => return p,
                _ => {}
            };

            return Self::generate_fib(p - 1) + Self::generate_fib(p - 2);
        }
    }
}

/**
 * Recursive + Memoezation
 */
mod impl_2 {
    use super::*;
    use std::collections::HashMap;

    pub struct FibGenerator;

    impl Fibunacci for FibGenerator {
        fn generate_fib(p: u64) -> u64 {
            match p {
                0 | 1 => return p,
                _ => {}
            }

            let mut map = HashMap::new();
            Self::generate_fib_helper(p, &mut map)
        }
    }

    impl FibGenerator {
        fn generate_fib_helper(p: u64, map: &mut HashMap<u64, u64>) -> u64 {
            match p {
                0 | 1 => return p,
                _ => {}
            }

            if map.contains_key(&p) {
                *map.get(&p).unwrap()
            } else {
                let (p_1, p_2) = (
                    Self::generate_fib_helper(p - 1, map),
                    Self::generate_fib_helper(p - 2, map),
                );
                map.insert(p, p_1 + p_2);
                *map.get(&p).unwrap()
            }
        }
    }
}

/**
 * Iterative
 */
mod impl_3 {
    use super::*;
    pub struct FibGenerator;

    impl Fibunacci for FibGenerator {
        fn generate_fib(p: u64) -> u64 {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_1() {
        use impl_1::FibGenerator as G;

        let result = [
            G::generate_fib(0),
            G::generate_fib(1),
            G::generate_fib(2),
            G::generate_fib(3),
            G::generate_fib(4),
            G::generate_fib(5),
            G::generate_fib(6),
            G::generate_fib(7),
        ];
        assert_eq!(result, [0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_impl_2() {
        use impl_2::FibGenerator as G;

        let result = [
            G::generate_fib(0),
            G::generate_fib(1),
            G::generate_fib(2),
            G::generate_fib(3),
            G::generate_fib(4),
            G::generate_fib(5),
            G::generate_fib(6),
            G::generate_fib(7),
        ];
        assert_eq!(result, [0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_impl_3() {
        use impl_3::FibGenerator as G;

        let result = [
            G::generate_fib(0),
            G::generate_fib(1),
            G::generate_fib(2),
            G::generate_fib(3),
            G::generate_fib(4),
            G::generate_fib(5),
            G::generate_fib(6),
            G::generate_fib(7),
        ];
        assert_eq!(result, [0, 1, 1, 2, 3, 5, 8, 13]);
    }
}
