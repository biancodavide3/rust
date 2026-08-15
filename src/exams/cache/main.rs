use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Condvar, Mutex};

// state is modeled either being computed by another thread or Ready with its value
enum State<V> {
    Computing,
    Ready(Arc<V>),
}

// a condvar for each Entry of the cache is needed because multiple
// threads may need to wait for a single entry
struct Entry<V> {
    state: Mutex<State<V>>,
    condvar: Condvar,
}

pub struct Cache<K, V> {
    map: Mutex<HashMap<K, Arc<Entry<V>>>>,
}

impl<K, V> Cache<K, V>
where K: Eq + Hash + Clone,
{
    pub fn get<F>(&self, k: K, f: F) -> Arc<V>
    where F: FnOnce(K) -> V,
    {
        // find the entry or create it with Computing state and return flag should_compute
        let (entry, should_compute) = {
            let mut map = self.map.lock().unwrap();
            match map.get(&k) {
                Some(entry) => (Arc::clone(entry), false),
                None => {
                    let entry = Arc::new(Entry {
                        state: Mutex::new(State::Computing),
                        condvar: Condvar::new(),
                    });
                    map.insert(k.clone(), Arc::clone(&entry));
                    (entry, true)
                }
            }
        };

        // if a thread was the one to insert a certain entry its execution of this method
        // will have should_compute = true it will do so and return the value
        if should_compute {
            let result = Arc::new(f(k));
            let mut state = entry.state.lock().unwrap();
            *state = State::Ready(Arc::clone(&result));
            entry.condvar.notify_all();
            return result;
        }

        // now all these other threads if they get here (i.e. should_compute = false)
        // will just wait for the computation of the first thread to be over before returning the result
        let mut state = entry.state.lock().unwrap();
        loop {
            match &*state {
                State::Ready(value) => {
                    return Arc::clone(value);
                }
                State::Computing => {
                    state = entry.condvar.wait(state).unwrap();
                }
            }
        }
    }
}