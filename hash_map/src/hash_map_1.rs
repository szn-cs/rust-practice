use std::cmp::Eq;
use std::collections::LinkedList;
use std::mem;

use datastructure::specification::hash_map::minimal::HashMap as Spec;

const INITIAL_CAPACITY: usize = 10;

pub trait Hashable {
    fn hash(&self) -> u64; // calculate hash code
}

// hash map implementation relying on double-ended queue
pub struct HashMap<K, V>
where
    K: Hashable + Eq + Clone,
    V: Clone,
{
    bucket_list: Vec<LinkedList<(K, V)>>,
    pub len: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hashable + Eq + Clone,
    V: Clone,
{
    // fn bucket(&self, key: K) -> usize; // helper to calculate hash code

    pub fn new() -> Self {
        Self {
            len: 0,
            bucket_list: vec![LinkedList::new(); INITIAL_CAPACITY],
        }
    }
}

impl<K, V> Spec<K, V> for HashMap<K, V>
where
    K: Hashable + Eq + Clone,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // resize if full/overloaded (e.g. 3/4 full)
        if self.len == 0 || self.len >= 3 * self.bucket_list.len() / 4 {
            self.extend();
        }
        assert!(self.len < self.bucket_list.len());

        let bucket = (key.hash() % self.bucket_list.len() as u64) as usize; // modulus guaranteed to be of usize following denominator
        let bucket = &mut self.bucket_list[bucket];
        // if present replace
        for &mut (ref k, ref mut v) in bucket.iter_mut() {
            if *k == key {
                let old_value = mem::replace(v, value);
                return Some(old_value);
            }
        }

        // otherwise insert new
        bucket.push_back((key, value));
        self.len += 1;

        None
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.len == 0 {
            return None;
        }

        let bucket = (key.hash() % self.bucket_list.len() as u64) as usize; // no truncation of remainder (guaranteed to be usize)
        let bucket = &mut self.bucket_list[bucket]; // linked list bucket

        for &mut (ref k, ref mut v) in bucket.iter_mut() {
            if k == key {
                return Some(v);
            }
        }

        None
    }

    #[allow(unused_variables)]
    fn remove(&mut self, key: &K) -> Option<V> {
        if self.len == 0 {
            return None;
        }

        let bucket = (key.hash() % self.bucket_list.len() as u64) as usize; // no truncation of remainder (guaranteed to be usize)
        let bucket = &mut self.bucket_list[bucket];

        let position = {
            let mut p = None;
            for (i, &mut (ref k, _)) in bucket.iter_mut().enumerate() {
                if k == key {
                    p = Some(i);
                }
            }
            p
        };

        position.map(|p| {
            let (_, v) = bucket.remove(p);
            v
        })
    }

    fn extend(&mut self) {
        let new_capacity = self.bucket_list.capacity() * 2;
        let mut new_bucket_list: Vec<LinkedList<(K, V)>> = Vec::with_capacity(new_capacity);

        for _ in (0..new_capacity) {
            new_bucket_list.push(LinkedList::new());
        }

        for (key, value) in self
            .bucket_list
            .iter_mut()
            .flat_map(|bucket| bucket.into_iter())
        {
            let bucket = (key.hash() % self.bucket_list.capacity() as u64) as usize;
            let bucket = &mut new_bucket_list[bucket];
            bucket.push_back((key, value));
        }

        std::mem::replace(&mut self.bucket_list, new_bucket_list);
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hashable + Eq + Clone,
    V: Clone,
{
    fn get_mut_idiomatic(&mut self, key: &K) -> Option<&mut V> {
        if self.len == 0 {
            return None;
        }

        let bucket = (key.hash() % self.bucket_list.len() as u64) as usize; // no truncation of remainder (guaranteed to be usize)
        let bucket = &mut self.bucket_list[bucket]; // linked list bucket

        bucket
            .iter_mut()
            .find(|&(ref k, _)| k == key)
            .map(|&mut (_, ref mut v)| v)
    }

    fn insert_idiomatic(&mut self, key: K, value: V) -> Option<V> {
        // resize if full/overloaded (e.g. 3/4 full)
        if self.len == 0 || self.len >= 3 * self.bucket_list.len() / 4 {
            self.extend();
        }
        assert!(self.len < self.bucket_list.len());

        let bucket = (key.hash() % self.bucket_list.len() as u64) as usize; // modulus guaranteed to be of usize following denominator
        let bucket = &mut self.bucket_list[bucket];

        // alternative implementation
        /*{
            if false {
                // another implementation could be to mem::swap value
                if let Some(v) = bucket
                    .iter_mut()
                    .find(|&(ref k, _)| k == &key)
                    .map(|&mut (_, ref mut v)| v)
                {
                    Some(std::mem::replace(v, value))
                } else {
                    bucket.push_back((key, value));
                    None
                }
            } else {
                None
            };
        }*/
        let mut old_value = None;
        if let Some(i) = bucket
            .iter_mut() // &mut (K,V)
            .position(|&mut (ref k, _)| k == &key)
            .map(|i| i)
        {
            let (_, v) = bucket.remove(i);
            self.len -= 1;
            old_value = Some(v);
        }

        bucket.push_back((key, value));
        self.len += 1;

        old_value
    }
}
