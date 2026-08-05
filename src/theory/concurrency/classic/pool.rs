use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>
    ) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let res = receiver
                    .lock()
                    .unwrap()
                    .recv();
                match res {
                    Ok(job) => {
                        println!("Worker {id} executing job");
                        job();
                    },
                    Err(_) => break
                }
            }
        });

        Worker {
            id,
            thread
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::new();

        for id in 0..size {
            workers.push(
                Worker::new(
                    id,
                    Arc::clone(&receiver)
                )
            );
        }

        ThreadPool {
            workers,
            sender
        }
    }

    fn execute<F>(&self, f: F)
    where F: FnOnce() + 'static + Send
    {
        let job = Box::new(f);
        self.sender
            .send(job)
            .unwrap();
    }
}

// example usage

pub fn main() {
    let pool = ThreadPool::new(4);
    for i in 0..8 {
        pool.execute(move || {
            println!("Task {i} started");
            thread::sleep(
                Duration::from_secs(1)
            );
            println!("Task {i} finished");
        });
    }
    thread::sleep(Duration::from_secs(5));
}