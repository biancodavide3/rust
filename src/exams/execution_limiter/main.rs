use std::sync::{Arc, Condvar, Mutex};

struct State {
    current: usize,
}

struct ExecutionLimiter {
    limit: usize,
    shared: Arc<(Mutex<State>, Condvar)>,
}

struct Permit<'a> {
    shared: &'a (Mutex<State>, Condvar),
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let (lock, cv) = self.shared;
        let mut state = lock.lock().unwrap();
        state.current -= 1;
        cv.notify_one();
    }
}

impl ExecutionLimiter {
    fn new(limit: usize) -> Self {
        Self {
            limit,
            shared: Arc::new((
                Mutex::new(State { current: 0 }),
                Condvar::new(),
            )),
        }
    }

    fn execute<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let (lock, cv) = &*self.shared;
        let mut state = lock.lock().unwrap();

        while state.current >= self.limit {
            state = cv.wait(state).unwrap();
        }

        state.current += 1;
        drop(state);

        let _permit = Permit {
            shared: &*self.shared,
        };

        f()
    }
}