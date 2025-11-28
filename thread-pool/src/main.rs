use crate::threadpool::ThreadPool;

mod threadpool;

fn main() {
    let thread_pool = ThreadPool::new(10);
    println!("created a thread pool");

    for i in 0..100 {
        thread_pool.spawn(move || println!("Task num {i}"));
    }

    println!("thread pool work done!")
}
