#[test]
fn test_zip_iterator() {
    let b = vec![1, 2, 3, 4, 5];
    let a = vec![11, 22, 33];

    let t = a.iter().zip(b.iter()).collect::<Vec<_>>();

    dbg!(t);
}

#[test]
fn test_reverse() {
    {
        let v = [1, 2, 3, 4, 5, 6];

        for e in v.iter().rev() {
            println!("{}", e);
        }
    }

    {
        let iter = (0..10);

        for e in iter.rev() {
            println!("{}", e);
        }
    }

    {
        let mut i = 0;
        for e in std::iter::from_fn(|| {
            i += 1;
            Some(2_u64.pow(i))
        })
        .take(10)
        .collect::<Vec<_>>()
        .iter()
        .rev()
        {
            println!("{}", e);
        }
    }
}
