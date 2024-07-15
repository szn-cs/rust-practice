use std::iter;

#[test]
fn pow2_range() {
    let mut counter = 1;
    let iterator = iter::from_fn(move || {
        let number = 2u64.pow(counter);
        counter += 1;
        Some(number)
    });

    for n in iterator.clone().take(7) {
        println!("{}", n);
    }

    let list = iterator.take(7).collect::<Vec<_>>();
    assert_eq!(list, [2, 4, 8, 16, 32, 64, 128]);
}

#[test]
fn rev_from_infinite_iter() {
    let iter = (0..).take(7).collect::<Vec<_>>().into_iter().rev();

    for e in iter {
        println!("{}", e);
    }
}
