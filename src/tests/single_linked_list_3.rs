use datastructure::single_linked_list_minimal::SingleLinkedList as L;
use specification::datastructure::linked_list::minimal::SingleLinkedList;

use std::option;

#[test]
fn append() {
    let mut l = L::<String>::new();
    l.push("str1".into());
    l.push("str2".into());
    l.push("str3".into());
    assert_eq!(l.len, 3);
}

#[test]
fn remove() {
    let mut l = L::<String>::new();
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

#[test]
fn traverse() {
    let v: Vec<String> = vec!["str1".into(), "str2".into(), "str3".into(), "str4".into()];

    let mut l = L::<String>::new();
    l.push("str1".into());
    l.push("str2".into());
    l.push("str3".into());
    l.push("str4".into());

    for (i, e) in l.into_iter().enumerate() {
        assert_eq!(e, v[i]);
    }

    // for (i, e) in l.into_iter().enumerate() {
    //     assert_eq!(e, v[i]);
    // }
}
