#[cfg(test)]
mod tests {
    use super::*;

    pub fn print_2d_slices(v: &[&[i32]]) -> usize {
        for r in v.iter() {
            for c in r.iter() {
                print!("{c}");
            }
            println!(" ");
        }

        0
    }

    #[test]
    fn case_1() {
        const N: usize = 5;
        const M: usize = 3;

        let v = vec![vec![0; M]; N];
        let vec_of_slices: Vec<_> = v
            .as_slice()
            .iter()
            .map(std::vec::Vec::as_slice) // .map(|vector| vector.as_slice())
            .collect();
        let slices_of_slices = &vec_of_slices[..];

        print_2d_slices(slices_of_slices);

        let v = vec![0; M * N];
        let vec_of_slices: Vec<_> = v.as_slice().chunks(M).collect();
        let slices_of_slices = vec_of_slices.as_slice();

        print_2d_slices(slices_of_slices);
    }
}
