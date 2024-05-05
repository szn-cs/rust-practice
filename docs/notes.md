# Reference

std::rc::{Rc, Weak}; 
std::cell::{RefCell, Cell}; 
std::sync::{Mutex, MutexGuard, Arc}; 
std::boxed::Box; 
std::option::Option; 
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
    - .and_then
    - .clone
    - .collect

# Traits
std::borrow::Borrow; 
std::iter::{Iterator, IntoIterator};  

## Trait constraint
where V: ?Sized  // where V doesn't have to be sized, useful if V is used as reference &V 

# macros
panic!() / unreachable!()
todo!() / unimplemented!()


# other patterns
Option<NonNull<T>> // None = discriminant 0x0 value of NonNull


# tools
- cargo clippy
- cargo miri

# examples

- lifetime reference annotations
```
let ref = books.iter().next().unwrap(); 
std::mem::drop(books); 
// ref through lifetime annotation is tied/associated to the books lifetime. 
```

- destructure references
```
for (&k, &v) in &hash_map { // assume hash_map.iter() iterator returns type items of (&'a K, &'a V) where HashMap<'a, K, V> 
    match k { // note type of k is the value referred to by &k
        "str" => assert_eq!(v, 42); 
        _ => {}
    }
}
```

- bind to outer/whole value
```
match bucket.pop() { 
    x @ Some(_) => break x, 
    _ => {}
}
```