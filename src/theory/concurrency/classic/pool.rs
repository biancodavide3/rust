use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// simple thread pool that just executes tasks without any other special function

trait Executor {
    fn execute<F>(&self, f: F)
    where F: FnOnce() + 'static + Send;
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// worker creates its own thread internally and puts it on an infinite loop
// it gets job from the channel (receiver wrapped in mutex, arc)

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>
    ) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let res = receiver
                    .lock()
                    .unwrap()   // auto deref
                    .recv();
                match res { // match here instead to exit cleanly when there is no more jobs
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
            thread  // return join handle of internal thread for manipulation
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        // create the channel with the right primitives 
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        
        // create the workers
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
}

impl Executor for ThreadPool {
    fn execute<F>(&self, f: F)
    where F: FnOnce() + 'static + Send
    {
        // simply wrap the closure in our job type for dynamic dispatch and use the sender
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