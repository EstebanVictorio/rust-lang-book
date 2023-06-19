use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Task = Box<dyn FnOnce(usize) -> Result<(), std::io::Error> + Send + 'static>;

pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Task>>,
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        println!("Dropping sender, no more tasks will be accepted.");
        drop(self.sender.take());
        println!("Dropped sender and dropping all workers, no more tasks will be accepted beyond this point, \"main\" thread awaiting all current workers to finish.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // For this, thread needs to be an Option given that the worker pool would take ownership of the thread,
            // and we need to be able to take ownership back to join the thread, since the join() method takes ownership of its argument
            // So we use "take()" from the Option thread to take the value out of the Option which would be the handle of the thread, and then join the thread
            // to the main thread so it can finish its work. The main thread will wait for it to finish before continuing.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let resolved = receiver.lock().unwrap().recv();

            match resolved {
                Ok(task) => {
                    println!("Worker {} got a job; executing.", id);
                    match task(id) {
                        Ok(_) => {
                            println!("Worker {} finished successfully.", id);
                        }
                        Err(_) => {
                            println!("Worker {} failed to execute job.", id);
                        }
                    }
                }
                Err(_) => {
                    println!("Worker {} not capable to receive task, shutting down.", id);
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
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

        WorkerPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(usize) -> Result<(), std::io::Error> + Send + 'static,
    {
        let task = Box::new(f);
        self.sender.as_ref().unwrap().send(task).unwrap();
    }
}

#[cfg(test)]
mod tests {}
