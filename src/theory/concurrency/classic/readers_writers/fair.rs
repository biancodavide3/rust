use std::sync::{Condvar, Mutex};

// using a turnstile approach to assure fairness
// it basically acts as a door before a reader or writer can do any operation 

trait FairReadersWriters<T>: Send + Sync {
    fn read(&self) -> T;
    fn write(&self, value: T);
}

struct State {
    readers: usize,
    writer_active: bool,
}

struct ReaderWriter<T> {
    value: Mutex<T>,
    state: Mutex<State>,
    condvar: Condvar,
    turnstile: Mutex<()>
}

impl<T> ReaderWriter<T> {
    fn new(value: T) -> Self {
        Self {
            value: Mutex::new(value),
            state: Mutex::new(State {
                readers: 0,
                writer_active: false
            }),
            condvar: Condvar::new(),
            turnstile: Mutex::new(())
        }
    }
}

impl<T> FairReadersWriters<T> for ReaderWriter<T>
where T: Send + Sync + Clone {
    fn read(&self) -> T {
        let turn = self.turnstile.lock().unwrap();
        let mut state = self.state.lock().unwrap();
        while state.writer_active {
            state = self.condvar.wait(state).unwrap();
        }
        state.readers += 1;
        drop(state);
        drop(turn);

        let value = self.value.lock().unwrap().clone();

        let mut state = self.state.lock().unwrap();
        state.readers -= 1;

        if state.readers == 0 {
            self.condvar.notify_all();
        }

        value
    }

    fn write(&self, value: T) {
        let turn = self.turnstile.lock().unwrap();
        let mut state = self.state.lock().unwrap();
        while state.readers > 0 || state.writer_active {
            state = self.condvar.wait(state).unwrap();
        }
        state.writer_active = true;
        drop(state);
        drop(turn);

        *self.value.lock().unwrap() = value;

        let mut state = self.state.lock().unwrap();
        state.writer_active = false;

        self.condvar.notify_all();
    }
}

