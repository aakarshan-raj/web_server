use std::{
    sync::{
        self,
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = sync::mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = vec![];

        for id in 0..size {
            let thread_rx = Arc::clone(&receiver);
            threads.push(Worker::new(id, thread_rx))
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            let job = rx.lock().unwrap().recv().unwrap();
            println!("worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}
