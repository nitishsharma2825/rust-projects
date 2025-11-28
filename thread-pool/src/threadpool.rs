use std::{sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    jobs: Arc<Mutex<Vec<Job>>>,
    cond: Arc<Condvar>,
}

impl ThreadPool {
    pub fn new(num_threads: u32) -> ThreadPool {
        let mut workers = Vec::with_capacity(num_threads as usize);
        let jobs = Arc::new(Mutex::new(Vec::new()));
        let cond = Arc::new(Condvar::new());

        for _ in 0..num_threads {
            let j = jobs.clone();
            let c = cond.clone();
            workers.push(thread::spawn(move || worker_loop(j, c)));
        }

        ThreadPool { workers, jobs, cond }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // push job to queue
        let mut jobs = self.jobs.lock().unwrap();
        jobs.push(Box::new(f));

        // notify one worker
        self.cond.notify_one();
    }
}

fn worker_loop(queue: Arc<Mutex<Vec<Job>>>, cond: Arc<Condvar>) {
    loop {
        let mut jobs = queue.lock().unwrap();
        
        // check if there is a job, else wait
        while jobs.is_empty() {
            // release the mutex and sleep untill notify
            jobs = cond.wait(jobs).unwrap();
        }

        let job = jobs.pop().unwrap();
        job();
    }
}