mod impl_sync {
    use std::thread::{self, sleep};
    use std::time::Duration;

    fn do_work_1() {
        println!("work_1 done - thread #{}", thread::current().id().as_u64());
    }

    fn do_work_2() {
        sleep(Duration::from_millis(50));
        println!("work_2 done - thread #{}", thread::current().id().as_u64());
    }

    pub fn run_threads(n: usize) {
        use std::thread::{spawn, JoinHandle};

        let mut v: Vec<JoinHandle<_>> = vec![];

        for i in 0..n {
            let h = if i % 2 == 0 {
                spawn(&do_work_1)
            } else {
                spawn(&do_work_2)
            };

            println!("Running thread #{}", h.thread().id().as_u64());
            v.push(h);
        }

        for h in v {
            h.join().unwrap();
        }
    }

    pub fn run_scoped_threads(n: usize) {
        use std::thread::scope;

        let mut shared = 0;
        scope(|s| {
            for i in 0..n {
                let id;
                if i % 2 == 0 {
                    let h = s.spawn(&do_work_1);
                    id = h.thread().id().as_u64();
                    shared += 1;
                } else {
                    let h = s.spawn(&do_work_2);
                    id = h.thread().id().as_u64();
                    shared += 1;
                }
                println!(
                    "Running scoped thread #{} with shared value: {}",
                    id, shared
                );
            }
        })
    }
}

mod impl_async {
    use futures::future::join_all;
    use std::boxed::Box;
    use std::future::Future;
    use std::pin::Pin;

    async fn do_work_a() -> usize {
        println!("doing work A");
        1
    }

    async fn do_work_b() -> usize {
        println!("doing work B");
        2
    }

    enum WorkType {}

    // workaround current future implementation which distinguishes between some opaque Future types even if they "look the same" (error: "expected future, found a different future").
    async fn do_work(work_type: WorkType) {}

    pub async fn run_async_work(n: usize) -> usize {
        let mut v: Vec<Pin<Box<dyn Future<Output = usize>>>> = vec![]; // declaring type will implicitely cast Future using type inference & coercion.

        let mut r = 0;
        for i in 0..n {
            if i % 2 == 0 {
                let f = do_work_a();
                v.push(Box::pin(f));
            } else {
                let f = do_work_b();
                v.push(Box::pin(f));
            }
        }

        let vector_result = join_all(v).await;

        r += vector_result
            .into_iter()
            .reduce(|acc, cur| acc + cur)
            .unwrap();
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_thread() {
        impl_sync::run_threads(10);
        std::thread::sleep(std::time::Duration::from_millis(200));
        impl_sync::run_scoped_threads(10);
    }

    #[test]
    fn case_futures() {
        use futures::executor::block_on;

        let r = block_on(impl_async::run_async_work(10));
        assert_eq!(r, 15);
    }
}
