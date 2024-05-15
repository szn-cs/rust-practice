# Reference

std::rc::{Rc, Weak}; 
std::cell::{RefCell, Cell}; 
std::sync::{Mutex, MutexGuard, Arc}; 
std::boxed::Box; 
std::option::Option; 
std::ptr::NonNull; 
std::marker::PhantomData; 
std::mem::{replace, swap, take, size_of, align_of}

std::ops::{Fn, FnMut, FnOnce}
std::ops::{Drop}
std::cmp::{Eq, PartialEq, Ordering, Ord}

std::fmt::{Debug, Display, Formatter};

## Collections
std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
std::collections::hash_map::DefaultHasher; 


# API

std::vec::Vec
- .swap_remove
- .extend_from_slice

std::option::Option
- .take()

std::boxed::Box
- .into_raw() // consume
- .from_raw() // reclaim leaked memory
- .as_ptr_mut() // convert

std::ptr::NonNull
- .as_ptr() 
- .as_mut() 

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
    - .filter_map

slice
- .split_at_mut
- .clone_from_slice
- .sort_by
- .binary_search_by

ParialOrd: 
- .parial_cmp -> Ordering
Ord: 
- .cmp -> Ordering

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

- ignore/empty errors
```
fn do_something() -> Result<i32, ()> {
    if true { 
        Result::Ok(123)
    } else {
        Result::Err( () )
    }
}
```
- accept any error
```
fn do_something() -> Result<(), Box<dyn std::error::Error>> {
    if true { 
        Ok(())
    } else { 
        Err(format!("{}", "something"))
    }
}
```

- define function as params with static dispatch
```
fn func(f: impl Fn(i3s) -> i32) {
    f(10);
}
```
- using traits generics 
```
fn func<F>(f: F) where F: Fn()->() { 
    f();
}
```

- swap elements in vector
```
pub fn swap<T>(arr: &mut [T], i: usize, j: usize) {
    let (low, high) = match i.cmp(&j) {
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
        Ordering::Equal => return,
    };
    
    let (a, b) = arr.split_at_mut(high); // to allow multiple mutable references on different objects (assuming left/right disjoint)
    
    std::mem::swap(&mut a[low], &mut b[0]);
}
```

- interface for free-standing functions with as an extension approach 
```
trait Sorter { 
    fn sort<T>(slice: &mut [T]) where T: Ord; 
}

fn sort<T, S>(slice: &mut [T]) where T: Ord, S: Sorter { 
    S::sort(slice); // redirect input to specific implementation
}

fn main() { 
    struct StdSort; // define a custom implementation
    impl Sorter for StdSort { 
        fn sort<T>(slice: &mut [T]) where T: Ord { 
            slice.sort(); 
        }
    }

    let mut things = vec![4,3,2,1]; 
    sort::<_, StdSort>(&mut things); 
}
```

# notes: 
- shared ref, mut unique ref, owned. 
- std::vec::Vec -> pointer to contiguous mem, capacity, len; 
- std::collections::VecDeque -> growable ring buffer (vector wrapping around) with start & end pointers to specify initialized region. Can implement both stack and queue;
    - Disadv.: Defragmentation may lead to slightly less efficient caching. Has extra overhead for calculating every index (to take account for wrapping). Doesn't implement deref to slice because of fragmentation. 
- std::collecitons::HashMap -> sparsity property (remedied by resizing # of buckets); 
    - bucket_list: Vec<Vec<(K, V)>>; Vec<LinkedList<(K,V)>>; Linear probing Vec<(K, V)>
    - cryptographic hash to avoid dential of service attacks by generating requests of keys with conflicting hashes filling up the bucket with collisions. 
- std::collections::{HashSet, BTreeSet} implemented as HashMap/BTreeMap with keys as set type and empty values. 
    - eliminates duplicates
    - API: contains, intersection, union, bitwise operators (impl. using union/intersection funcs), etc.
-std::collections::BTreeMap are ordered.
    - Unlike binary tree (BSTs), BTree allow each node hold `B` values in a vector of capacity B = 6  (multiple values within each node). Reduces amount of metadata stored (e.g. less pointers). 
    - cache friendly compared to BST pointers or even if mapped as an array has random indexing pattern. May lead to more work to do mutation but cache efficient. Similar to m-way search tree but is more constrainted to fill at least half of the node is filled & leafs at same level.
    - O(log n) insertion/lookup; 
    - requires Ord orderable (not Hashable) for keys; 
    - The tree is usually fairly full, thus it in general for the same data requires less memeory than HashMaps (factor of 1.x). 
    - for splitting and appending can reuse existing allocations of the B vectors. Built down-up.
- std::collection::BinaryHeap collection for getting min/max out of the elements (priority queue). Unordered. implemented as flattened binary tree inside a vector, maintaining priority at index 0 with log(n) swaps. Allow duplicate elements.  
    - O(1) retreival/peak of priority element; O(log n) insertion/deletion; 
    - O(n) Heapify (traverse vector right-left and bubble down). 
    - O(n log n) Heap creation. 
    - O(log n) heapify down (doens't require entire tree heapify).
    - important to preserve complete binary tree property. 
    - heap sort: delete elements and place them in the freed cell will result in sorted elements.
