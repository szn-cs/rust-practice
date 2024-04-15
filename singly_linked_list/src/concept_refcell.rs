#![allow(unused_variables)]

use std::cell::RefCell;

pub fn interior_mutability_reference() {
    let c: RefCell<i32> = RefCell::new(10);
    {
        let r1 = c.borrow();
        let r2 = c.borrow();
        let r3 = c.borrow();

        assert!(c.try_borrow_mut().is_err());
    }
    {
        let m = c.borrow_mut();
        assert!(c.try_borrow().is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interior_mutability() {
        interior_mutability_reference();
    }
}
