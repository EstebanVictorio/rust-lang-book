use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct WorkerPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        Self {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        /// Create a new WorkerPool.
        /// If size is 0, then panic.
        assert!(size > 0);

        // We then create a way to communicate work among workers, so we create a shared channel via a channel + a mutex with atomic reference counting for safe thread sharing of the channel via the mutex.
        let (sender, receiver) = mpsc::channel::<mpsc::Receiver<Job>>();
        let mutex = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i));
        }

        WorkerPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
