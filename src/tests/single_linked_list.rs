use singly_linked_list::SingleLinkedList as L;

#[test]
fn append() {
    let mut l: L = L::new();
    l.append(1);
    l.append(2);
    l.append(3);

    assert_eq!(l.len, 3);
}

#[test]
fn pop() {
    let mut l: L = L::new();
    l.append(1);
    l.append(2);
    l.append(3);

    let v = l.pop();
    assert_eq!(v, Some(1));
}
