use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Task = Box<dyn FnOnce(usize) -> Result<(), std::io::Error> + Send + 'static>;

pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            todo!("Wrap Option for thread");
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Self {
        Self {
            id,
            thread: thread::spawn(move || loop {
                let task = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} starting task, running task...");
                match task(id) {
                    Ok(_) => println!("Worker {id} finished task successfully."),
                    Err(e) => println!("Worker {id} failed task with error: {e}"),
                }
            }),
        }
    }
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        /// Create a new WorkerPool.
        /// If size is 0, then panic.
        assert!(size > 0);

        // We then create a way to communicate work among workers, so we create a shared channel via a channel + a mutex with atomic reference counting for safe thread sharing of the channel via the mutex.
        let (sender, receiver) = mpsc::channel::<Task>();
        let mutex = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&mutex)));
        }

        WorkerPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(usize) -> Result<(), std::io::Error> + Send + 'static,
    {
        let task = Box::new(f);
        self.sender.send(task).unwrap();
    }
}

#[cfg(test)]
mod tests {}
