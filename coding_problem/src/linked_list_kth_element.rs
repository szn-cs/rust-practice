use std::collections::LinkedList;
use std::ptr::NonNull;

type N = Option<NonNull<Node<i32>>>;
struct Node<T> {
    next: N,
    prev: N,
    element: T,
}

// get k-th last element from linked list
fn kth_element(list: &mut LinkedList<i32>, k: usize) -> Option<i32> {
    // add edge cases
    if list.len() < k {
        return None;
    }

    let p = list as *mut _ as *mut u8;

    let p = list as *mut LinkedList<i32> as *mut u8;
    let head = unsafe { (p.offset(0) as *mut N).as_mut().unwrap() };
    let tail = unsafe {
        (p.offset(std::mem::size_of::<N>() as isize) as *mut N)
            .as_mut()
            .unwrap()
    };

    let mut p1 = *head;
    let mut p2 = *head;
    let mut k = k;
    while match p2 {
        Some(_) if k > 0 => {
            k -= 1;
            p2 = unsafe { p2.unwrap().as_mut() }.next;
            true
        }
        Some(_) => {
            p1 = unsafe { p1.unwrap().as_mut() }.next;
            p2 = unsafe { p2.unwrap().as_mut() }.next;

            true
        }
        None => {
            // p1 = unsafe { p1.unwrap().as_mut() }.next;
            false
        }
    } {}

    let element = unsafe { p1.unwrap().as_ref() }.element;
    Some(element)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut l = LinkedList::from([1, 2, 3, 4, 5, 6, 7, 8]);
        let k = kth_element(&mut l, 3);

        assert_eq!(k, Some(6));
    }

    #[test]
    fn case_2() {
        let mut l = LinkedList::from([1, 2, 3, 4, 5]);
        let k = kth_element(&mut l, 5);

        assert_eq!(k, Some(1));
    }

    #[test]
    fn case_3() {
        let mut l = LinkedList::from([1, 2, 3, 4, 5]);
        let k = kth_element(&mut l, 6);

        assert_eq!(k, None);
    }
}
