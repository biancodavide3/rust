use std::collections::VecDeque;
use std::sync::{mpsc, Condvar, Mutex};

pub trait BoundedBuffer<T>: Send + Sync {
    fn push(&self, item: T);
    fn pop(&self) -> T;
}

// implementation 1

// use a queue and 2 condvar

pub struct CondvarBuffer<T> {
    capacity: usize,
    queue: Mutex<VecDeque<T>>,
    not_empty: Condvar,
    not_full: Condvar,
}

impl<T> CondvarBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            queue: Mutex::new(VecDeque::new()), // add mutex for synchronizing different threads on the queue
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }
}

impl<T: Send + Sync> BoundedBuffer<T> for CondvarBuffer<T> {
    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        while queue.len() == self.capacity {    // wait if the queue is full before pushing
            queue = self.not_full.wait(queue).unwrap();
        }
        queue.push_back(item);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() { // wait if the queue is empty before popping
            queue = self.not_empty.wait(queue).unwrap();
        }
        let value = queue.pop_front().unwrap();
        self.not_full.notify_one();
        value
    }
}

// implementation 2

// using sync channel, it is much simpler

pub struct ChannelBuffer<T> {
    sender: mpsc::SyncSender<T>,
    receiver: Mutex<mpsc::Receiver<T>>, // add Mutex to have multiple receivers
}

impl<T> ChannelBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) =
            mpsc::sync_channel(capacity);   // sync_channel is a bounded channel per se
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

impl<T: Send + Sync> BoundedBuffer<T> for ChannelBuffer<T> {
    fn push(&self, item: T) {
        self.sender.send(item).unwrap();
    }

    fn pop(&self) -> T {
        self.receiver
            .lock()
            .unwrap()
            .recv()
            .unwrap()
    }
}

pub fn main() {
    
}