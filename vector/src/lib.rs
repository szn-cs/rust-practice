pub mod impl_1 {
    pub struct Vector<T> {
        buffer: *mut T,  // buf
        capacity: usize, // cap
        length: usize,   // len
    }
}

pub mod impl_2 {
    use std::option::Option;
    use std::ptr::NonNull;

    pub struct Vector<T> {
        buffer: Option<NonNull<T>>,
        capacity: usize,
        length: usize,
    }
}
