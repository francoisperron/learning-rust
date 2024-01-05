use std::sync::{Arc, mpsc, Mutex};
use crate::threads::message::Message;
use crate::threads::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::Execute(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down thread pool");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }

        for worker in &mut self.workers {
            worker.terminate();
        }
    }
}