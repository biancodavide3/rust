use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

pub trait Resource<T: Send> {
    fn get(&self) -> &T;
}

pub fn make_resource_pool<T: Send>(items: Vec<T>) -> impl ResourcePool<T> {
    let capacity = items.capacity();
    MyResourcePool {
        shared: Arc::new((
            Mutex::new(PoolState {
                available: items
            }),
            Condvar::new()
        )),
        capacity,
    }
}

pub trait ResourcePool<T: Send> {
    // numero totale di elementi gestiti dal pool
    fn capacity(&self) -> usize;

    // preleva un elemento dal pool e lo consegna al chiamante. Se nessun
    // elemento e' disponibile, blocca il chiamante senza consumare cicli di cpu
    // finche' una risorsa non viene rilasciata.
    fn acquire(&self) -> impl Resource<T>;

    // variante con attesa limitata: come acquire, ma se non ottiene un
    // elemento entro timeout rinuncia e restituisce None. L'attesa non deve
    // consumare CPU.
    fn acquire_timeout(&self, timeout: Duration) -> Option<impl Resource<T>>;
}

struct PoolState<T> {
    available: Vec<T>,
}

struct MyResource<T: Send> {
    value: Option<T>,
    shared: Arc<(Mutex<PoolState<T>>, Condvar)>,
}

impl<T: Send> Resource<T> for MyResource<T> {
    fn get(&self) -> &T {
        self.value.as_ref().unwrap()
    }
}

impl<T: Send> Drop for MyResource<T> {
    fn drop(&mut self) {
        let value = self.value.take().unwrap();
        let (mutex, condvar) = &*self.shared;
        let mut state = mutex.lock().unwrap();
        state.available.push(value);
        condvar.notify_one();
    }
}

struct MyResourcePool<T: Send> {
    shared: Arc<(Mutex<PoolState<T>>, Condvar)>,
    capacity: usize,
}

impl<T: Send> ResourcePool<T> for MyResourcePool<T> {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn acquire(&self) -> impl Resource<T> {
        let shared_clone = self.shared.clone();
        let (mutex, condvar) = &*shared_clone;
        let mut state = mutex.lock().unwrap();
        while state.available.is_empty() {
            state = condvar.wait(state).unwrap();
        }
        let value = state.available.pop().unwrap();
        MyResource {
            value: Some(value),
            shared: Arc::clone(&self.shared),
        }
    }

    fn acquire_timeout(&self, timeout: Duration) -> Option<impl Resource<T>> {
        let shared_clone = self.shared.clone();
        let (mutex, condvar) = &*shared_clone;
        let mut state = mutex.lock().unwrap();
        while state.available.is_empty() {
            let (guard, result) =
                condvar.wait_timeout(state, timeout).unwrap();
            state = guard;
            if result.timed_out() {
                return None;
            }
        }
        let value = state.available.pop().unwrap();
        Some(MyResource {
            value: Some(value),
            shared: Arc::clone(&self.shared),
        })
    }
}