// Recursive O(2^ m*n)T;
mod impl_1 {
    pub fn unique_path(N: usize, M: usize) -> usize {
        // base case
        if N == 1 || M == 1 {
            // if first row or first column
            1
        } else {
            unique_path(N - 1, M) + unique_path(N, M - 1)
        }
    }
}

// DP O(m*n)
mod impl_2 {
    pub fn unique_path(N: usize, M: usize) -> usize {
        let mut v = vec![vec![0; M]; N];

        // set 1st column/row to 1
        for i in 0..N {
            v[i][0] = 1;
        }
        for j in 0..M {
            v[0][j] = 1;
        }

        for i in 1..N {
            for j in 1..M {
                v[i][j] = v[i - 1][j] + v[i][j - 1];
            }
        }

        v[N - 1][M - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        use impl_1::unique_path;

        const N: usize = 5;
        const M: usize = 3;

        let result = unique_path(N, M);
        assert_eq!(result, 15);
    }

    #[test]
    fn case_2() {
        use impl_2::unique_path;

        const N: usize = 5;
        const M: usize = 3;

        let result = unique_path(N, M);
        assert_eq!(result, 15);
    }
}
