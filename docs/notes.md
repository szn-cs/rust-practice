# Reference

std::rc::{Rc, Weak}; 
std::cell::{RefCell, Cell}; 
std::sync::{Mutex, MutexGuard, RwLock, Arc, mpsc}; 
    - std::thread
std::boxed::Box;
std::option::Option; 
std::ptr::{NonNull, null()/null_mut(), drop_in_place(*mut T)}; //  matching *const T and *mut T types 
std::marker::{PhantomData, Copy, Send /*mutable access*/, Sync /*immutable access*/, Sized}; 
std::mem::{replace, swap, take, size_of, align_of, drop}
std::alloc::Layout::{new, array} // layout of mem passed to allocator
std::alloc::{dealloc(*mut u8, Layout), alloc(Layout)}

std::ops::{Fn, FnMut, FnOnce}
std::ops::{Drop}
std::cmp::{Eq, PartialEq, Ordering, Ord}
-   std::cmp::min;

std::fmt::{Debug, Display, Formatter};

## Collections
std::collections::{VecDeque, LinkedList, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
std::collections::hash_map::DefaultHasher; 
// implement indexing
std::ops::{Index, IndexMut}; 

# API

std::string::String
- .push_str(&mut self, &str)
- .push(&mut self, char)


std::vec::Vec
- .extend_from_slice
- .extend
- .swap_remove(usize)
- .remove(usize) // i.e. remove shift
std::collections::VecDequeue
- .make_contiguous()

std::option::Option
- .take()

std::boxed::Box
- .into_raw() // consume
- .from_raw() // reclaim leaked memory
- .as_ptr_mut() // convert
boxed slice: 
- .into_boxed_slice()


std::ptr::NonNull
- .as_ptr() -> *mut T
- .as_ref()/.as_mut()
- .write()

raw pointers: *const/*mut T
- .offset(i) 
- .write => std::ptr::write(*mut T, T)
- .read()
- .is_null()
- .drop_in_place() => std::ptr::drop_in_place
- .copy_nonovelapping => std::ptr::copy_nonovelapping(*const T, *mut T, usize)
- .copy => std::ptr::copy(...)

conversion: 
- .try_into().expect("")

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
    - .fold
    - .repeat
    - .take
    - .chain
    - .for_each
std::iter::from_fn
    - .take()
    - .take_while()

slice
- .split_at_mut
- .copy_from_slice
- .clone_from_slice
- .sort_by
- .sort_by_key
- .binary_search_by
- .rotate_right 
- slice::from_raw_slices(*const T, usize)

ParialOrd: 
- .parial_cmp -> Ordering
Ord: 
- .cmp -> Ordering

# Traits
std::borrow::Borrow; 
std::iter::{Iterator, IntoIterator};  

## Trait constraint
where V: ?Sized  // where V doesn't have to be sized, useful if V is used as reference &V 

# primitive
- .ceil/.floor()    => f64/f32
- .round() 

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
- for a sharable type to be sync it has to be send. Rules: if variable can be mutably referenced (& ops like move ownership, drop, mutate) from a thread other than the creating one => Send; If for a variable is allowed to make immutable parallel access from several threads => Sync. https://www.youtube.com/watch?v=eRxqX5_UxaY 
- &mut T isn't allowed to be aliased. 
- variane := defines mechanism to devise wether one type can be downgraded into another, taking into account lifetimes of the parameters, with relations of subtypes.
    - It defines which lifetimes can be downgraded in place of arguemnts or upgraded for return parameters in functions for example. 
- division of integers without convertion: let (remaining, fraction) = (end * 1000 / 2 % 1000, end / 2);
