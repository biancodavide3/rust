use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Task {
    deadline: Instant,
    id: usize,
    function: Box<dyn FnOnce() + Send + 'static>,
}

// BinaryHeap is a max-heap
// reverse the ordering so that the task with the earliest
// deadline is considered the greatest element.
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .deadline
            .cmp(&self.deadline)
            .then_with(|| other.id.cmp(&self.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline && self.id == other.id
    }
}

impl Eq for Task {}

struct State {
    tasks: BinaryHeap<Task>,
    closed: bool,
    drop_pending_tasks: bool,
    next_id: usize,
}

pub struct DelayedExecutor {
    shared: Arc<(Mutex<State>, Condvar)>,
    worker: Option<thread::JoinHandle<()>>,
}

impl DelayedExecutor {
    pub fn new() -> Self {
        let shared = Arc::new((
            Mutex::new(State {
                tasks: BinaryHeap::new(),
                closed: false,
                drop_pending_tasks: false,
                next_id: 0,
            }),
            Condvar::new(),
        ));
        let worker_shared = Arc::clone(&shared);
        let worker = thread::spawn(move || {
            let (mutex, condvar) = &*worker_shared;
            loop {
                let task = {
                    let mut state = mutex.lock().unwrap();
                    loop {
                        // If the executor is closed and we must
                        // discard pending tasks, we can terminate.
                        if state.closed && state.drop_pending_tasks {
                            state.tasks.clear();
                            break None;
                        }
                        // No tasks.
                        if state.tasks.is_empty() {
                            if state.closed {
                                // Closed and no pending tasks.
                                break None;
                            }
                            // Wait until a new task arrives or close()
                            // notifies us.
                            state = condvar.wait(state).unwrap();
                            continue;
                        }
                        // There is at least one task.
                        let deadline = state.tasks.peek().unwrap().deadline;
                        let now = Instant::now();
                        if deadline <= now {
                            // The earliest task is ready.
                            break state.tasks.pop();
                        }
                        // The earliest task is not ready yet.
                        //
                        // Sleep until its deadline, but wake up earlier
                        // if a new task is inserted or close() is called.
                        let timeout = deadline.duration_since(now);
                        state = condvar
                            .wait_timeout(state, timeout)
                            .unwrap()
                            .0;
                    }
                };
                match task {
                    Some(task) => {
                        // IMPORTANT:
                        // The mutex is NOT held while executing the task.
                        (task.function)();
                    }
                    None => break,
                }
            }
        });
        Self {
            shared,
            worker: Some(worker),
        }
    }

    pub fn execute<F>(&self, f: F, delay: Duration) -> bool
    where F: FnOnce() + Send + 'static,
    {
        let (mutex, condvar) = &*self.shared;
        let mut state = mutex.lock().unwrap();
        if state.closed {
            return false;
        }
        let deadline = Instant::now() + delay;
        let id = state.next_id;
        state.next_id += 1;
        state.tasks.push(Task {
            deadline,
            id,
            function: Box::new(f),
        });
        // Wake the worker because the new task may have an earlier
        // deadline than the one it was previously waiting for.
        condvar.notify_one();
        true
    }

    pub fn close(&self, drop_pending_tasks: bool) {
        let (mutex, condvar) = &*self.shared;
        let mut state = mutex.lock().unwrap();
        // Once closed, execute() will return false.
        state.closed = true;
        state.drop_pending_tasks = drop_pending_tasks;
        if drop_pending_tasks {
            state.tasks.clear();
        }
        // Wake the worker so it can observe the shutdown.
        condvar.notify_one();
    }
}

impl Drop for DelayedExecutor {
    fn drop(&mut self) {
        let (mutex, condvar) = &*self.shared;
        {
            let mut state = mutex.lock().unwrap();
            state.closed = true;
            state.drop_pending_tasks = true;
            state.tasks.clear();
            condvar.notify_one();
        }
        // If a task is currently executing, join() waits for it
        // to finish. We never try to interrupt it.
        if let Some(worker) = self.worker.take() {
            worker.join().unwrap();
        }
    }
}