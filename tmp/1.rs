std::option::Option; 
std::iter::Iterator; 

pub mod spec { 
    pub trait SingleLinkedList : IntoIterator { 
        type Item; 
    
        pub fn push(&mut self, v: Self::Item); 
    
        pub fn pop(&mut self) -> Option<Self::Item>; 
    }
}

type Link = todo!(); 

pub struct SingleLinkedList : spec::SingleLinkedlist { 
    pub len: usize, 
    head: Option<Link>
}

impl spec::SingleLinkedList for SingleLinkedList { 
    type Item = i32; 
    
    fn push(&mut self, v: Self::Item) { 
        0 
    }

     fn pop(&mut self) -> Option<Self::Item> { 
        None
    }
}

impl IntoIterator for SingleLinkedList { 
    type Item = i32; 
    type IntoIter = Iter<i32>; 

    fn into_iter(self) -> Self::IntoIter {
        Iter::<i32>::new()
    }
}

