use std::thread::{JoinHandle, spawn};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};

pub struct ThreadPool{
    size: usize,
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{
            size: size,
            workers,
            sender,
        }
    }

    pub fn size(&self) -> &usize {
        &self.size
    }

    pub fn execute<F>(self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}