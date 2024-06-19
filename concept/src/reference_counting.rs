use std::rc::{Rc, Weak};

fn reference_counting() {
    let x = Rc::new(4);
    let y = Rc::clone(&x);

    assert_eq!(Rc::into_inner(y), None); // consumes the clone
    assert_eq!(Rc::into_inner(x), Some(4)); // strong reference counter = 1  thus suceeds
}

fn rc_weak() {
    use std::ptr;

    let strong: Rc<String> = Rc::new("hello".to_owned());
    let weak: Weak<String> = Rc::downgrade(&strong);
    // Both point to the same object
    let t: &String = &*strong; // coerced into *const String
    let k: *const String = weak.as_ptr();
    assert!(ptr::eq(t, k));
    // The strong here keeps it alive, so we can still access the object.
    assert_eq!("hello", unsafe { &*weak.as_ptr() });

    drop(strong);
    // But not any more. We can do weak.as_ptr(), but accessing the pointer would lead to
    // undefined behaviour.
    // assert_eq!("hello", unsafe { &*weak.as_ptr() });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        reference_counting();
    }

    #[test]
    fn test_rc_weak() {
        rc_weak();
    }
}
