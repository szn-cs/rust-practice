/**
 * Given a matrix representing water/land, find the number of islands present
 * - represent 1 as land, 0 as water
 */

/**
 *  O(n)T; O(log n)S if stack DS is used; n = # of elements in grid[r][c]
 */
mod impl_1 {
    pub fn count_islands(grid: &mut Vec<Vec<usize>>) -> usize {
        // basic verification of proper matrix
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let ROW = grid.len();
        let COL = grid[0].len();

        let mut counter = 0;

        for r in 0..ROW {
            for c in 0..COL {
                // count number of islands when encountering the first land portion
                if is_land(grid[r][c]) {
                    counter += 1;
                    dfs_sink_island(grid, r, c);
                }
            }
        }

        counter
    }

    fn is_land(value: usize) -> bool {
        if value == 1 {
            true
        } else {
            false
        }
    }

    fn dfs_sink_island(grid: &mut Vec<Vec<usize>>, r: usize, c: usize) {
        grid[r][c] = 1;

        let ROW = grid.len();
        let COL = grid[0].len();

        // visit each neighbour
        for direction in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let r = r as i32 + direction.0;
            let c = c as i32 + direction.1;

            if r < 0 || r >= ROW as i32 {
                continue;
            }

            if c < 0 || c >= COL as i32 {
                continue;
            }

            let r = r.unsigned_abs() as usize;
            let c = c.unsigned_abs() as usize;

            if !is_land(grid[r][c]) {
                continue;
            }

            // checked_add
            dfs_sink_island(grid, r, c);
        }
    }
}

/**
 * non-mutable grid; rely on seen hashset datastructure with no recursion
 * O(n)T; O(n)S
 */
mod impl_2 {
    use std::collections::HashSet;

    #[derive(PartialEq)]
    pub enum Terrain {
        Land,
        Water,
    }

    pub fn count_islands(grid: &Vec<Vec<Terrain>>) -> usize {
        // edge case checks
        let ROW = grid.len();
        if ROW == 0 {
            return 0;
        }
        let COL = grid[0].len();
        if COL == 0 {
            return 0;
        }

        let mut counter = 0;
        let mut seen = HashSet::<(usize, usize)>::new(); // store seen indecies

        for r in 0..ROW {
            for c in 0..COL {
                if grid[r][c] == Terrain::Land && !seen.contains(&(r, c)) {
                    counter += 1;

                    dfs_visit_island(grid, &mut seen, r, c);
                }
            }
        }

        counter
    }

    fn dfs_visit_island(
        grid: &Vec<Vec<Terrain>>,
        seen: &mut HashSet<(usize, usize)>,
        r: usize,
        c: usize,
    ) {
        let (ROW, COL) = (grid.len(), grid[0].len());
        let mut stack = vec![(r, c)];

        // visit neighbor using dfs
        while let Some(current) = stack.pop() {
            seen.insert((r, c));

            'd: for direction in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let neighbor = (
                    current.0 as i32 + direction.0,
                    current.1 as i32 + direction.1,
                );

                if neighbor.0 < 0 || neighbor.1 < 0 {
                    continue 'd;
                }

                let neighbor = (
                    neighbor.0.unsigned_abs() as usize,
                    neighbor.1.unsigned_abs() as usize,
                );

                // check bounds
                if neighbor.0 >= ROW || neighbor.1 >= COL {
                    continue 'd;
                }

                if grid[neighbor.0][neighbor.1] == Terrain::Water {
                    continue;
                }

                stack.push((neighbor.0, neighbor.1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_impl_1() {
        use impl_1::count_islands;

        let mut grid = vec![vec![1, 1, 0, 1], vec![1, 0, 0, 1], vec![0, 1, 0, 1]];
        let r = count_islands(&mut grid);

        assert_eq!(r, 3);
    }

    #[test]
    fn case_impl_2() {
        use impl_2::{count_islands, Terrain::*};

        let mut grid = vec![
            vec![Land, Land, Water, Land],
            vec![Land, Water, Water, Land],
            vec![Water, Land, Water, Land],
        ];
        let r = count_islands(&grid);

        assert_eq!(r, 3);
    }
}
