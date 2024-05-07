# Reference

std::rc::{Rc, Weak}; 
std::cell::{RefCell, Cell}; 
std::sync::{Mutex, MutexGuard, Arc}; 
std::boxed::Box; 
std::option::Option; 
std::ptr::NonNull; 
std::marker::PhantomData; 
std::mem::{replace, swap, take}

std::ops::{Fn, FnMut, FnOnce}
std::ops::{Drop}
std::cmp::{Eq, PartialEq}

std::fmt::{Debug, Display, Formatter};

## Collections
std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
std::collections::hash_map::DefaultHasher; 


# API
Box::into_raw() // consume
Box::from_raw() // reclaim leaked memory
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

# crates
use rand::Rng; let x = rand::random::<usize>(); 

# other patterns
- Option<NonNull<T>> // None = discriminant 0x0 value of NonNull
- leak functions: leaves allocated memory on heap never deallocating them and returns *mut pointer. 

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

- variable binding in patterns
```
for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() { // ref and ref mut are not being matched against but define how to bind the variables created. 
    if ekey == &key {
        return Some(mem::replace(evalue, value));
    }
}
```

- mutability of reference vs binding: 
```
let mut x = &mut y; // both value is mutable reference and x variable binding is mutable which can change location pointing to. 
```

# notes: 
- shared ref, mut unique ref, owned. 
- std::vec::Vec -> pointer to contiguous mem, capacity, len; 
- std::collections::VecDeque -> growable ring buffer (vector wrapping around) with start & end pointers to specify initialized region. Can implement both stack and queue;
    - Disadv.: Defragmentation may lead to slightly less efficient caching. Has extra overhead for calculating every index (to take account for wrapping). Doesn't implement deref to slice because of fragmentation. 
- std::collecitons::HashMap -> sparsity property (remedied by resizing # of buckets); 
    - bucket_list: Vec<Vec<(K, V)>>; Vec<LinkedList<(K,V)>>; Linear probing Vec<(K, V)>