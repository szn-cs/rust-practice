# Reference

std::rc::{Rc, Weak}; 
std::cell::{RefCell, Cell}; 
std::sync::{Mutex, MutexGuard, Arc}; 
std::boxed::Box; 
std::option::Option; 
std::iter::{Iterator, IntoIterator}; 
std::ptr::NonNull; 
std::marker::PhantomData; 

std::ops::{Fn, FnMut, FnOnce}
std::ops::{Drop}

std::fmt::{Display, Formatter};

## Collections
std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};


# API
Box::into_raw() // consume
Box::from_raw()
Box::as_ptr_mut() // convert

# other patterns
Option<NonNull<T>> // None = discriminant 0x0 value of NonNull
