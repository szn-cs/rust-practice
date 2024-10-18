mod spec {
    pub trait Queue {
        fn enqueue(&mut self, e: i32); // adds the element to the queue
        fn dequeue(&mut self) -> Option<i32>; // removes element from queue
    }
}

pub struct Queue {
    enqueue_stack: Vec<i32>,
    dequeue_stack: Vec<i32>,
    len: usize,
}

impl Queue {
    fn new() -> Self {
        Self {
            enqueue_stack: vec![],
            dequeue_stack: vec![],
            len: 0,
        }
    }
}

impl spec::Queue for Queue {
    fn enqueue(&mut self, e: i32) {
        self.enqueue_stack.push(e);
        self.len += 1;
        dbg!(&self.enqueue_stack);
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }

        if self.dequeue_stack.is_empty() {
            // move elements from other stack
            for e in self.enqueue_stack.drain(..).rev() {
                self.dequeue_stack.push(e);
            }
        }

        self.len -= 1;
        let v = self.dequeue_stack.pop();
        dbg!(&self.dequeue_stack);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spec::Queue as _Queue;

    #[test]
    fn enqueue_dequeue() {
        let mut q = Queue::new();
        let mut v = vec![];

        for i in 1..=4 {
            q.enqueue(i);
        }

        for _ in 1..=2 {
            v.push(q.dequeue().unwrap());
        }

        for i in 5..=10 {
            q.enqueue(i);
        }

        for _ in 1..=2 {
            v.push(q.dequeue().unwrap());
        }

        q.enqueue(11);

        for _ in 1..=7 {
            v.push(q.dequeue().unwrap());
        }

        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(q.dequeue(), None);
    }
}
