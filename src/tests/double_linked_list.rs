use specification::datastructure::linked_list::minimal::DoubleLinkedList as Spec;
use single_linked_list::double_linked_list::DoubleLinkedList as LinkedList;

#[test]
fn comprehensive_double_linked_list() {
    let mut l = LinkedList::<i32>::new();

    l.push_front(3);
    l.push_back(5);
    l.push_front(2);
    l.push_back(6);
    l.push_front(1);
    l.push_back(7);

    assert_eq!(l.pop_front(), Some(1));
    assert_eq!(l.pop_back(), Some(7));

    l.traverse(|v: &mut i32| {
        *v = *v + 10;
    });
}
