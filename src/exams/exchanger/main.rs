use std::sync::{Arc, Condvar, Mutex};

struct State<T> {
    value: Option<T>,
    waiting: bool,
}

struct Exchanger<T> {
    shared: Arc<(Mutex<State<T>>, Condvar)>,
}

impl<T> Exchanger<T> {
    fn new() -> Self {
        Self {
            shared: Arc::new((
                Mutex::new(State {
                    value: None,
                    waiting: false,
                }),
                Condvar::new(),
            )),
        }
    }

    fn exchange(&self, t: T) -> T {
        let (lock, cv) = &*self.shared;
        let mut state = lock.lock().unwrap();
        if !state.waiting {
            state.waiting = true;
            state.value = Some(t);
            while state.waiting {
                state = cv.wait(state).unwrap();
            }
            state.value.take().unwrap()
        }
        else {
            let other = state.value.take().unwrap();
            state.value = Some(t);
            state.waiting = false;
            cv.notify_one();
            other
        }
    }
}