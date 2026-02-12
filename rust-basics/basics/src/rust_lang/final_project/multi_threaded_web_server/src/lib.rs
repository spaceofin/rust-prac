#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(path_statements)]

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

#[derive(Debug)]
pub struct PoolCreationError {
    pub details: String,
}

impl PoolCreationError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(|| {
            receiver;
        });
        Self { id, thread }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::new(
                "ThreadPool size must be greater than 0",
            ))
        }
        Ok(Self::new(size))
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}