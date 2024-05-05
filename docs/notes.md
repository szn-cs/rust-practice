# Reference

std::rc::{Rc, Weak}; 
std::cell::{RefCell, Cell}; 
std::sync::{Mutex, MutexGuard, Arc}; 
std::boxed::Box; 
std::option::Option; 
std::iter::{Iterator, IntoIterator}; 
std::ptr::NonNull; 
std::marker::PhantomData; 
std::mem::{replace}

std::ops::{Fn, FnMut, FnOnce}
std::ops::{Drop}

std::fmt::{Display, Formatter};

## Collections
std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
std::collections::hash_map::DefaultHasher; 


# API
Box::into_raw() // consume
Box::from_raw()
Box::as_ptr_mut() // convert

std::vec::Vec
- .swap_remove
-  

std::iter::Iterator
    - .extend
    - .flat_map
    - .drain 
    - .map
    - .find
    - .retain 
    - .position
    - 

# other patterns
Option<NonNull<T>> // None = discriminant 0x0 value of NonNull
 

# tools
- cargo clippy
- cargo miri

# examples
```
let ref = books.iter().next().unwrap(); 
std::mem::drop(books); 
// ref through lifetime annotation is tied/associated to the books lifetime. 
```