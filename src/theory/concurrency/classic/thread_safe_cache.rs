use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::thread;

// Key needs Eq + Hash to work with hashmap
// note that in get contains &key because we don't own
// while in insert we want to pass ownership

// in our trait implementation .get() returns option but,
// we don't own the value
// so use cloned() to get a clone

// that's why V requires Clone

trait Cache<K, V>
where
    K: Eq + Hash,
{
    fn get(&self, key: &K) -> Option<V>;
    fn insert(&self, key: K, value: V);
    fn contains(&self, key: &K) -> bool;
}

struct ThreadSafeCache<K, V> {
    data: Mutex<HashMap<K, V>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl<K, V> Cache<K, V> for ThreadSafeCache<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    fn insert(&self, key: K, value: V) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    fn contains(&self, key: &K) -> bool {
        let data = self.data.lock().unwrap();
        data.contains_key(key)
    }
}

pub fn main() {
    let cache = Arc::new(
        ThreadSafeCache::<String, String>::new()
    );

    let mut handles = Vec::new();

    for id in 0..5 {
        let cache = Arc::clone(&cache);

        handles.push(thread::spawn(move || {
            let key = format!("user-{id}");

            cache.insert(
                key.clone(),
                format!("User {id}")
            );

            if let Some(value) = cache.get(&key) {
                println!(
                    "Thread {id}: {value}"
                );
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}