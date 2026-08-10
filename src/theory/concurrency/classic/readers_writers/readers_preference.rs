use std::sync::{Condvar, Mutex};

// the problem
// multiple readers can access a shared resource at the same time
// a single writer can access a shared resource at some time
// only one of the 2 above possibilities at the same time

// 3 solutions:

// 1) readers preference
// a writer could be starved if readers keep coming

// we use 2 different mutex to protect synchronization state State
// and the actual resource T

// make use of a condvar to notify threads

// when reading acquire the state
// wait if there is a writer active (writer_active = true)
// increase the count of readers
// NOTE: immediately release state lock so other threads can see
// acquire resource lock, read it, decrease readers count and return the resource

// when writing acquire the state
// wait if there is another writer active or the count of readers is > 0
// when the lock is acquired set writer_active = true
// immediately release state lock again
// write to the resource, set writer_active = false exit

trait ReadersPreference<T>: Send + Sync {
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
        }
    }
}

impl<T> ReadersPreference<T> for ReaderWriter<T>
where T: Send + Sync + Clone
{
    fn read(&self) -> T {
        let mut state = self.state.lock().unwrap();
        while state.writer_active {
            state = self.condvar.wait(state).unwrap();
        }

        // note auto deref from MutexGuard<State>
        state.readers += 1;
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
        while state.writer_active || state.readers > 0 {
            state = self.condvar.wait(state).unwrap();
        }
        state.writer_active = true;
        drop(state);

        let mut old = self.value.lock().unwrap();
        *old = value;

        let mut state = self.state.lock().unwrap();
        state.writer_active = false;

        self.condvar.notify_all();
    }
}