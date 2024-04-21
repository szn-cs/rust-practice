use std::cell::Cell;

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
        *s = 40; // Error: cannot borrow as mutable more than one time
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interior_mutability() {
        interior_mutability();
    }
}
