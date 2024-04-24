use datastructure::specification::linked_list::minimal::SingleLinkedList;
use single_linked_list::sinlge_linked_list_pointer::SingleLinkedList as L;

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

    let iter = (&mut l).into_iter();
    for e in iter {
        e.insert_str(0, "_");
        e.remove(0);
    }

    for e in &mut l {
        println!("{}", e);
    }

    {
        let mut count = 0;
        let iter = l.into_iter();
        for (i, e) in iter.enumerate().inspect(|_| count += 1) {
            assert_eq!(e, v[i]);
        }

        assert_eq!(count, v.len());
    }
    // for (i, e) in l.into_iter().enumerate() {
    //     assert_eq!(e, v[i]);
    // }
}
