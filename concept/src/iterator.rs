#[test]
fn test_zip_iterator() {
    let b = vec![1, 2, 3, 4, 5];
    let a = vec![11, 22, 33];

    let t = a.iter().zip(b.iter()).collect::<Vec<_>>();

    dbg!(t);
}
