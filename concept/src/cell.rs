#![allow(unused_variables)]

use std::cell::Cell;
use std::cell::RefCell;

pub fn interior_mutability() {
    {
        let c: Cell<i32> = Cell::<i32>::new(10);
        println!("{}", c.get());
        println!("{}", c.take());
        println!("{}", c.get());

        c.set(20);
        println!("{}", c.get());
    }

    {
        let mut c: Cell<i32> = Cell::<i32>::new(10);
        let mut r: &mut i32 = c.get_mut(); // compile time borrowing
        *r = 40;
        println!("{}", r);
    }

    {
        let mut c: Cell<i32> = Cell::<i32>::new(10);
        let s = c.get_mut();
        *s = 20;
        println!("{}", s);
        let t = c.get_mut();
        *t = 30;
        println!("{}", t);
        // *s = 40; // Error: cannot borrow as mutable more than one time
        // println!("{}", s);
    }
}

fn interior_mutability_references() {
    let c = RefCell::new(10_f64); // NOTE: (a) variable binding is not mutable.
    {
        {
            let b = c.borrow();
            println!("{b:.3}");
            let e = c.borrow();
            println!("{e:.3}");
        }

        let mut m = c.borrow_mut(); // (a) yet it is possible to borrow it mutably.
        *m = 5 as f64;
        println!("{m:.3}");

        {
            // let p = c.borrow(); // compiler doesn't catch the borrow violation; runtime error already borrowed mutably
        }
    }
    {
        println!("{}", c.try_borrow_mut().is_ok()); // true (2)
        let mut t = c.borrow_mut();
        println!("{}", c.try_borrow_mut().is_ok()); // false (2)
    }
    println!("{}", c.into_inner());
}

pub fn interior_mutability_reference_ref() {
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
    fn test_interior_mutability_ref() {
        interior_mutability_reference_ref();
    }

    #[test]
    fn test_interior_mutability() {
        interior_mutability();
    }

    #[test]
    fn test_interior_mutability_references() {
        interior_mutability_references();
    }
}
