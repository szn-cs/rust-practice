use singly_linked_list_2::SinglyLinkedList as L;

use std::option;

#[test]
fn append() {
    let mut l: L = L::new();
    l.push("str1".into());
    l.push("str2".into());
    l.push("str3".into());
    assert_eq!(l.len, 3);
}

#[test]
fn remove() {
    let mut l: L = L::new();
    l.push("str1".into());
    l.push("str2".into());
    l.push("str3".into());

    let v = l.pop();
    assert_eq!(v, Some("str1".into()));

    let v = l.pop();
    assert_eq!(v, Some("str2".into()));

    l.pop(); // "str3"
    let v = l.pop();
    assert_eq!(v, Option::None);

    let v = l.pop();
    assert_eq!(v, Option::None);
}
