use binary_heap::BinaryHeap;
use specification::datastructure::binary_heap::minimal::BinaryHeap as Spec;

#[test]
fn binary_heap_usage() {
    let mut l = BinaryHeap::with_comparator(|a: &i32, b: &i32| a > b);

    l.push(10);
    l.push(30);
    l.push(2);

    assert_eq!(l.len(), 3);

    assert_eq!(l.pop(), Some(30));
    l.push(11);
    assert_eq!(l.pop(), Some(11));
    assert_eq!(l.pop(), Some(10));
    assert_eq!(l.pop(), Some(2));

    assert!(l.len() == 0);
}
