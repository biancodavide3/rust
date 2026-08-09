use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// a rate limiter limits N request over some time T
// while a semaphore just limits N request concurrently
// so in a semaphore as soon as one permit becomes available a new request can start
// while in a rate limiter everyone just waits and is coordinated

// use a token bucket style limiter

// state which is how many tokens are available and last refill are protected by mutex since
// they should be read by different threads
// while capacity and refill_rate are internal properties of the Bucket

// a rate limiter acquires requests by calculating the number of new tokens as T * refill rate
// where T is the time between now when the requests is happening and the last refill

// refill rate is tokens/second

// if those tokens >= 1 we refill the bucket (max refill = capacity)
// and update state

// if there is at least 1 token we use it for a request
// else we just wait 1 / refill rate with wait_timeout in Condvar

// so we for example if refill rate is 3 => we have a new request ever 1/3 seconds

trait RateLimiter: Send + Sync {
    fn acquire(&self);
}

struct TokenBucket {
    capacity: usize,
    refill_rate: usize,
    state: Mutex<BucketState>,
    available: Condvar,
}

struct BucketState {
    tokens: usize,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: usize, refill_rate: usize) -> Self {
        Self {
            capacity,
            refill_rate,
            state: Mutex::new(BucketState {
                tokens: capacity,
                last_refill: Instant::now(),
            }),
            available: Condvar::new(),
        }
    }
}

impl RateLimiter for TokenBucket {
    fn acquire(&self) {
        let mut state = self.state.lock().unwrap();

        loop {
            let now = Instant::now();
            let elapsed = now.duration_since(state.last_refill);

            let new_tokens =
                elapsed.as_secs_f64() * self.refill_rate as f64;

            if new_tokens >= 1.0 {
                state.tokens = std::cmp::min(
                    self.capacity,
                    state.tokens + new_tokens as usize,
                );

                state.last_refill = now;
            }

            if state.tokens > 0 {
                state.tokens -= 1;
                return;
            }

            // No token available: wait before checking again.
            let wait_time =
                Duration::from_secs_f64(
                    1.0 / self.refill_rate as f64
                );

            state = self
                .available
                .wait_timeout(state, wait_time)
                .unwrap()
                .0;
        }
    }
}

pub fn main() {
    let limiter = Arc::new(
        TokenBucket::new(3, 1)
    );

    let mut handles = Vec::new();

    for id in 0..10 {
        let limiter = Arc::clone(&limiter);

        handles.push(thread::spawn(move || {
            limiter.acquire();

            println!(
                "Thread {id} performing request"
            );
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}