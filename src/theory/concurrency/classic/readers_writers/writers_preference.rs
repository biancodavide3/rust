// 2) writer preference

// the only difference from readers preference is introducing another count
// waiting_writer in state that the readers have to wait out to 0
// before reading so they are the ones starved in case

// this works because we increment waiting_writer for every writer that comes
// before it starts the wait and after decrease to 0
// so if he did not wait at all it's 2 wasted operations

use std::sync::{Condvar, Mutex};

trait WriterPreference<T> {
    fn read(&self) -> T;
    fn write(&self, value: T);
}

struct State {
    readers: usize,
    writer_active: bool,
    waiting_writers: usize,
}

struct ReaderWriter<T> {
    state: Mutex<State>,
    value: Mutex<T>,
    condvar: Condvar,
}

impl<T> ReaderWriter<T> {
    fn new(value: T) -> Self {
        Self {
            state: Mutex::new(State {
                readers: 0,
                writer_active: false,
                waiting_writers: 0
            }),
            value: Mutex::new(value),
            condvar: Condvar::new(),
        }
    }
}

impl<T> WriterPreference<T> for ReaderWriter<T>
where T: Send + Sync + Clone
{
    fn read(&self) -> T {
        let mut state = self.state.lock().unwrap();
        while state.writer_active || state.waiting_writers > 0 {
            state = self.condvar.wait(state).unwrap();
        }

        (*state).readers += 1;
        drop(state);

        let value = self.value.lock().unwrap().clone();

        let mut state = self.state.lock().unwrap();
        state.readers -= 1;

        if state.readers == 0 {
            self.condvar.notify_all();
        }

        value
    }

    fn write(&self, value: T) {
        let mut state = self.state.lock().unwrap();

        state.waiting_writers += 1;

        while state.writer_active || state.readers > 0 {
            state = self.condvar.wait(state).unwrap();
        }

        state.waiting_writers -= 1;
        state.writer_active = true;
        drop(state);

        *self.value.lock().unwrap() = value;

        let mut state = self.state.lock().unwrap();
        state.writer_active = false;
        self.condvar.notify_all();
    }
}