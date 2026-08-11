// evolution of the worker pool pattern
// where a thread if it has finished its job can steal
// another one from the queue of another thread
// so a Mutex<VecDeque<Job>>

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct WorkQueue {
    jobs: Mutex<VecDeque<Job>>
}

impl WorkQueue {
    fn new() -> Self {
        Self {
            jobs: Mutex::new(VecDeque::new())
        }
    }

    fn push(&self, job: Job) {
        self.jobs.lock().unwrap().push_back(job);
    }

    fn pop(&self) -> Option<Job> {
        self.jobs.lock().unwrap().pop_back()
    }

    fn steal(&self) -> Option<Job> {
        self.jobs.lock().unwrap().pop_front()
    }
}

struct Worker {
    id: usize,
    queue: Arc<WorkQueue>,
    workers: Arc<Vec<Arc<WorkQueue>>>,
}

impl Worker {
    fn run(&self) {
        loop {
            if let Some(job) = self.queue.pop() {
                println!("worker {} executing own job", self.id);
                job();
                continue;
            }

            // now to try to steal a job
            // note auto deref with iter() and enumerate()

            let mut stolen = None;

            for (id, queue) in self.workers.iter().enumerate() {
                if id == self.id {
                    continue;
                }

                if let Some(job) = queue.steal() {
                    stolen = Some(job);
                    println!("worker {} stole from worker {}", self.id, id);
                    break;
                }
            }
            match stolen {
                Some(job) => job(),
                None => break,
            }
        }
    }
}

trait Executor {
    fn execute<F>(&self, workers: usize, f: F)
    where F: FnOnce() + Send + 'static;
}

struct WorkStealingPool {
    queues: Arc<Vec<Arc<WorkQueue>>>
}

impl WorkStealingPool {
    fn new(size: usize) -> Self {
        let mut queues = Vec::new();
        for _ in 0..size {
            queues.push(Arc::new(WorkQueue::new()))
        }
        Self {
            queues: Arc::new(queues)
        }
    }

    // batch executor
    // enqueue all work before starting workers
    
    fn start(&self) {
        let mut handles = Vec::new();

        for id in 0..self.queues.len() {
            let queue = Arc::clone(&self.queues[id]);
            let workers = Arc::clone(&self.queues);

            handles.push(thread::spawn(move || {
                Worker {
                    id,
                    queue,
                    workers,
                }
                    .run();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

impl Executor for WorkStealingPool {
    fn execute<F>(&self, worker: usize, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.queues[worker].push(Box::new(f));
    }
}