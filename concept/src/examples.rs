// - lifetime reference annotations
fn e1() {
    let ref = books.iter().next().unwrap(); 
    std::mem::drop(books); 
    // ref through lifetime annotation is tied/associated to the books lifetime. 
}

// - destructure references
fn e2() {
    for (&k, &v) in &hash_map { // assume hash_map.iter() iterator returns type items of (&'a K, &'a V) where HashMap<'a, K, V> 
        match k { // note type of k is the value referred to by &k
            "str" => assert_eq!(v, 42); 
            _ => {}
        }
    }
}

// - bind to outer/whole value
fn e3() {
    match bucket.pop() { 
        x @ Some(_) => break x, 
        _ => {}
    }
}

// - variable binding in patterns
fn e4() {
    for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() { // ref and ref mut are not being matched against but define how to bind the variables created. 
        if ekey == &key {
            return Some(mem::replace(evalue, value));
        }
    }
}

// - mutability of reference vs binding: 
{
    let mut x = &mut y; // both value is mutable reference and x variable binding is mutable which can change location pointing to. 
}

// - ignore/empty errors
{
    fn do_something() -> Result<i32, ()> {
        if true { 
            Result::Ok(123)
        } else {
            Result::Err( () )
        }
    }
}
// - accept any error
```
fn do_something() -> Result<(), Box<dyn std::error::Error>> {
    if true { 
        Ok(())
    } else { 
        Err(format!("{}", "something"))
    }
}
```

// - define function as params with static dispatch (trait object resolved statically)
```
fn func(f: impl Fn(i3s) -> i32) {
    f(10);
}
```
// - using traits generics 
```
fn func<F>(f: F) where F: Fn()->() { 
    f(10);
}
```
// - trait object dynamically resolved 
```
fn func<F>(f: dyn Fn()->()) { 
    f(10);
}
```


// - swap elements in vector
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

// - interface for free-standing functions with as an extension approach 
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

// - std::mem::copy of slice to another
```
    fn f<T: Copy>(s1: &mut [T], s2: &mut [T]) { 
        let s1 = vec![T::default(); 5]; 
        let s2 = vec![1,2,3,4,5];

        s1.copy_from_slice(s2);        
    }


    // another way
    for (dst, src) in dst.iter_mut().zip(src) { *dst = *src }

```