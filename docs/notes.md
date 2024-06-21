std::fmt::{Debug, Display, Formatter};
std::ops::{Fn, FnMut, FnOnce}
-   std::cmp::{min/max};
std::marker::{PhantomData, Copy, Send /*mutable access*/, Sync /*immutable access*/, Sized}; 
std::mem::{replace, swap, take, size_of, align_of, drop}
std::alloc::Layout::{new, array} // layout of mem passed to allocator
std::alloc::{dealloc(*mut u8, Layout), alloc(Layout)}
std::borrow::Borrow; 
std::iter::{Iterator, IntoIterator};  
std::ops::{Drop, Deref<Target = T>}
std::cmp::{Eq, PartialEq, Ordering, Ord}

# f64/f32 primitive
- .ceil/.floor()  
- .round() 
- .sqrt()  

## std::string::String
- .push_str(&mut self, &str)
- .push(&mut self, char)

## std::option::Option
- .take()
- .is_some/is_none() -> bool

## std::boxed::Box
- .into_raw() // consume
- .from_raw() // reclaim leaked memory
- .as_ptr_mut() // convert
- leak functions: leaves allocated memory on heap never deallocating them and returns *mut pointer. 
boxed slice: 
- .into_boxed_slice()

# pointers
- std::ptr::eq(*const T, *const T) -> bool
- std::ptr::{NonNull, null()/null_mut(), drop_in_place(*mut T)}; //  matching *const T and *mut T types 

## std::ptr::NonNull
- Option<NonNull<T>> // None = discriminant 0x0 value of NonNull
- .as_ptr() -> *mut T
- .as_ref()/.as_mut()
- .write()

## *const/*mut T
- .offset(i) 
- .write => std::ptr::write(*mut T, T)
- .read()
- .is_null()
- .drop_in_place() => std::ptr::drop_in_place
- .copy_nonovelapping => std::ptr::copy_nonovelapping(*const T, *mut T, usize)
- .copy => std::ptr::copy(...)
___
</br>

# std::cell::{Cell, RefCell, Ref<'b, T>, RefMut<'b, T>}
- provide interior mutability where it is not allowed; allows to imulate field-level mutability.

## Ref
    - Ref::clone(&Ref<'b, T>)
## std::cell::Cell
    - .get(&self) -> T
    - .set(&self, T)
    - misc: 
        - .replace(&self, T) -> T
        - .into_inner(self) -> T
        - .take(&self) -> T // leaving default in place
        - .get_mut(&mut self) -> &mut T  // defeats purpose; use RefCell instead
        - .as_ptr(&self) -> *mut T
## std::cell::RefCell
    - .borrow/borrow_mut(&self)  → Ref/RefMut<'_, T> 
        - .try_borrow/try_borrow_mut(&self) → Result<Ref/RefMut<'_, T>, BorrowError/BorrowMutError> 
    - misc:
        - .into_inner(self)                     // consumes wrapper, returning value
        - .take(&self)                          // leave default instead
        - .replace(&self, T)                    // corresponds to std::mem::replace
        - .get_mut(&mut self) -> &mut T         // statically checked only possible in some cases for performance
        - .as_ptr(&self) -> *mut T

# std::rc::{Rc, Weak}
- Rc<RefCell<T>>    =>    Arc<Mutex<T>> 
- Rc owning pointer  -downgrade->  Weak non-owning pointer
    - mutual owning references prevent deallocating either Rc.  

## std::rc::Rc
    - Rc::clone()
    - Rc::downgrade(&Rc<T>) -> Weak<T>  
    - misc:
        - Rc::strong_count/weak_count(&Rc<T>) -> usize
        - Rc::as_ptr(&Rc<T>) -> *const T
            - Rc::get_mut(&mut value) -> Option<&mut T>                                 // succeeds on single strong reference
                - Rc::make_mut(&mut Rc<T>) -> &mut T                                    // clones values if more than 1 reference; deassociates all Weak references
        - Rc::into_inner(Rc<T>) -> Option<T>    ==   Rc::try_unwrap(Rc<T>).ok()         // succeeds on single strong reference
            - Rc::try_unwrap(Rc<T>) -> Result<T, Rc<T>>                             
            - Rc::unwrap_or_clone(Rc<T>) -> T
        - Rc::into_raw(Rc<T>) -> *const T                                               // consumes Rc       
        - Rc::from_raw(*const T) -> Rc<T>
        - Rc::increment_strong_count/decrement_strong_count(*const T)
        - Rc::ptr_eq(&Rc<T>, &Rc<T>) -> bool 

## std::rc::Weak
    - .upgrade(&self) -> Option<Rc<T>>
    - misc: 
        - .as_ptr(&self) -> *const T
        - .into_raw(self) -> *const T
        - Weak::from_raw(*const T) -> Weak<T>
        - .ptr_eq(&self, &Weak<T>) -> bool
        - .weak_count/strong_count(&self) -> usize

# std::sync::{Arc, Weak}
- similar to std::rc::{Rc, Weak}; 

# std::sync::{Mutex, MutexGuard, RwLock, mpsc}

# std::thread::{park/unpark, scope, spawn, sleep}

# std::collections::{VecDeque, LinkedList, BinaryHeap, HashMap, BTreeMap, HashSet, BTreeSet};
- std::collections::hash_map::DefaultHasher; 
- std::ops::{Index, IndexMut};  // implement indexing

## std::vec::Vec
    - .push(&mut self, T)
    - .pop(&mut self) -> Option<T>
    - misc: 
        - .insert(&mut self, usize, T)              // shift insert
        - .remove(&mut self, usize) -> T                 //  shift remove
            - .swap_remove(&mut self, usize) -> T
        - .extend
            - .extend_from_slice

## std::collections::VecDeque
    - push_back, push_front, pop_back, pop_front 
    - misc: 
        - front_mut/back_mut

## std::collections::LinkedList
    - push_back, push_front, pop_back, pop_front
    - misc: 
        - front_mut/back_mut
        - .remove
        - [insert] = split + push_back + append 

## std::collections::BinaryHeap
    - push, pop, peak_mut
    - misc: 
        - .into_vec

## std::collections::{HashMap, BTreeMap}
    - insert, remove, get_mut
    - misc: 
        - contains_key
        - keys, values 
        - from 

## std::collections::{HashSet, BTreeSet}
    - insert, remove, contains

## slice
- .split_at_mut
- .copy_from_slice
- .clone_from_slice
- .sort_by
- .sort_by_key
- .binary_search_by
- .rotate_right 
- slice::from_raw_slices(*const T, usize)

## std::collections::VecDequeue
- .make_contiguous()


___ 
</br>


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

ParialOrd: 
- .parial_cmp -> Ordering
Ord: 
- .cmp -> Ordering

# macros
panic!() / unreachable!()
todo!() / unimplemented!()

# crates
```
use rand::Rng; 
let x = rand::random::<usize>(); 
```

# tools
- cargo clippy
- cargo miri

# notes: 
- owned vs shared ref and mut unique ref;
- Interior mutability: modifiable even behind shred-reference.  
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
- where V: ?Sized  // where V doesn't have to be sized, useful if V is used as reference &V 
- `let title_text = title.text().trim();`: FAILS because nothing binds text() String to context, thus trim() references temporary value in statement.