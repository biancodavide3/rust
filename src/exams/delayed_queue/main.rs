use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Instant;

struct Element<T: Send> {
    value: T,
    instant: Instant,
}

impl<T: Send> Ord for Element<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.instant.cmp(&self.instant)
    }
}

impl<T: Send> PartialOrd for Element<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Send> PartialEq for Element<T> {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}

impl<T: Send> Eq for Element<T> {}

struct DelayedQueue<T: Send> {
    shared: Arc<(Mutex<BinaryHeap<Element<T>>>, Condvar)>,
}

impl<T: Send> DelayedQueue<T> {
    fn new() -> Self {
        let shared = Arc::new((
            Mutex::new(BinaryHeap::<Element<T>>::new()),
            Condvar::new(),
        ));
        Self { shared }
    }

    fn offer(&self, t: T, i: Instant) {
        let (mutex, condvar) = &*self.shared;
        let mut queue = mutex.lock().unwrap();
        queue.push(Element {
            value: t,
            instant: i,
        });
        condvar.notify_all();
    }

    fn take(&self) -> Option<T> {
        let (mutex, condvar) = &*self.shared;
        let mut queue = mutex.lock().unwrap();
        loop {
            if queue.is_empty() {
                return None;
            }
            let instant = queue.peek().unwrap().instant;
            if instant <= Instant::now() {
                return Some(queue.pop().unwrap().value);
            }
            let duration = instant.duration_since(Instant::now());
            queue = condvar.wait_timeout(queue, duration).unwrap().0;
        }
    }

    fn size(&self) -> usize {
        let (mutex, _) = &*self.shared;
        let queue = mutex.lock().unwrap();
        queue.len()
    }
}