use crate::threads::message::Message;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Execute(job) => {
                    println!("Worker {id} got a job; executing.");
                    job()
                }
                Message::Terminate => {
                    println!("Worker {id} terminating.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }

    pub fn terminate(&mut self) {
        println!("Shutting down worker {}", self.id);

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
