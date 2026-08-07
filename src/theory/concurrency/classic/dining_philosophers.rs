use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

// there are 5 philosopher sitting at a table
// between each philosopher there is a fork
// each thinks -> grabs forks -> eats
// the rule: a philosopher must take both forks next to him to eat

// we need to avoid deadlock by breaking one of the 4 conditions:
// 1. mutual exclusion (only one philosopher can hold a fork)
// 2. hold and wait (e.g. hold left fork while waiting for right)
// 3. no preemption (can't steal a fork)
// 4. circular wait (p0 waits p1 that waits p2...)

// first define common interfaces

type Fork = Arc<Mutex<usize>>;

#[derive(Clone)]
struct Philosopher {
    id: usize,
    name: String,
}

impl Philosopher {
    fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    fn think(&self) {
        println!("{} is thinking", self.name);
        thread::sleep(Duration::from_millis(100));
    }

    fn eat(&self) {
        println!("{} is eating", self.name);
        thread::sleep(Duration::from_millis(100));
    }
}


// strategy interface
// every solution to dining philosophers implements this

trait DiningStrategy: Send + Sync {
    fn dine(
        &self,
        philosopher: Philosopher,
        left: Fork,
        right: Fork,
    );
}


// solution 1:
// always acquire forks in a fixed order
// prevents circular wait -> no deadlock
// requires that resources can be ordered

struct OrderedStrategy;

impl DiningStrategy for OrderedStrategy {
    fn dine(
        &self,
        philosopher: Philosopher,
        left: Fork,
        right: Fork,
    ) {
        for _ in 0..3 {

            philosopher.think();

            // extract fork ids before locking
            // Arc clone allows us to reorder ownership
            let (first, second) =
                if *left.lock().unwrap() < *right.lock().unwrap() {
                    (left.clone(), right.clone())
                } else {
                    (right.clone(), left.clone())
                };

            let _first_fork = first.lock().unwrap();

            println!(
                "{} acquired first fork",
                philosopher.name
            );

            let _second_fork = second.lock().unwrap();

            println!(
                "{} acquired second fork",
                philosopher.name
            );

            philosopher.eat();

            // mutex guards are dropped here
            // forks become available
        }
    }
}

// solution 2:
// try-lock retry
// tries to acquire the lock if possible otherwise fails immediately
// it breaks condition hold and wait
// downside is possible starvation

struct TryLockStrategy;

impl DiningStrategy for TryLockStrategy {
    fn dine(&self,
            philosopher: Philosopher,
            left: Fork,
            right: Fork) {
        loop {
            philosopher.think();

            let _left = left.try_lock();
            match _left {
                Ok(value) =>
                    println!("{} acquired first fork", philosopher.name),
                Err(err) => continue
            }

            let _right = right.try_lock();
            match _right {
                Ok(value) =>
                    println!("{} acquired first fork", philosopher.name),
                Err(err) => continue
            }

            philosopher.eat();

            break;
        }
    }
}


// table owns all forks and starts philosophers
struct Table<S>
where
    S: DiningStrategy,
{
    forks: Vec<Fork>,
    strategy: Arc<S>,
}

impl<S> Table<S>
where
    S: DiningStrategy + 'static,
{
    fn new(size: usize, strategy: S) -> Self {
        let mut forks = Vec::new();

        for id in 0..size {
            forks.push(
                Arc::new(
                    Mutex::new(id)
                )
            );
        }

        Self {
            forks,
            strategy: Arc::new(strategy),
        }
    }

    fn run(&self, philosophers: Vec<Philosopher>) {
        let mut handles = Vec::new();

        let size = self.forks.len();

        for philosopher in philosophers {

            let left =
                Arc::clone(
                    &self.forks[philosopher.id]
                );

            let right =
                Arc::clone(
                    &self.forks[(philosopher.id + 1) % size]
                );

            let strategy =
                Arc::clone(&self.strategy);

            handles.push(
                thread::spawn(move || {
                    strategy.dine(
                        philosopher,
                        left,
                        right,
                    );
                })
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

// solution 3: waiter
// introduce a central coordinator ( Waiter )
// with limited a limited number of permits ( semaphore like ) and a condvar
// available when a permit is free

struct Waiter {
    permits: Mutex<usize>,
    available: Condvar,
}


impl Waiter {
    fn new(number: usize) -> Self {
        Self {
            permits: Mutex::new(number),
            available: Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut permits =
            self.permits.lock().unwrap();
        while *permits == 0 {
            permits =
                self.available
                    .wait(permits)
                    .unwrap();
        }
        *permits -= 1;
    }

    fn release(&self) {
        let mut permits =
            self.permits.lock().unwrap();
        *permits += 1;
        self.available.notify_one();
    }
}


struct WaiterStrategy {
    waiter: Arc<Waiter>,
}


impl WaiterStrategy {

    fn new(number_of_philosophers: usize) -> Self {
        // allow N-1 philosophers to compete
        Self {
            waiter: Arc::new(
                Waiter::new(
                    number_of_philosophers - 1
                )
            ),
        }
    }
}


impl DiningStrategy for WaiterStrategy {

    fn dine(
        &self,
        philosopher: Philosopher,
        left: Fork,
        right: Fork,
    ) {

        for _ in 0..3 {
            philosopher.think();

            self.waiter.acquire();

            let _left =
                left.lock().unwrap();

            let _right =
                right.lock().unwrap();

            philosopher.eat();

            self.waiter.release();
        }
    }
}


pub fn main() {
    let philosophers = vec![
        Philosopher::new(0, "Plato"),
        Philosopher::new(1, "Aristotle"),
        Philosopher::new(2, "Socrates"),
        Philosopher::new(3, "Descartes"),
        Philosopher::new(4, "Kant"),
    ];

    let table = Table::new(
        5,
        OrderedStrategy
    );

    let table2 = Table::new(
        5,
        TryLockStrategy
    );

    // table.run(philosophers.clone());
    table2.run(philosophers.clone());
}