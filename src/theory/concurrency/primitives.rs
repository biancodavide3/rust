use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn main() {
    // thread();
    arc_mutex_condvar();
    channel();
}

fn thread() {
    // simplified signature
    /*
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
     */

    let handle: JoinHandle<()> = thread::spawn(|| {
        println!("Hello from worker");
    });
    // spawn tells the os to create a new thread

    println!("Hello from main thread");
    // handle.join().unwrap(); // try commenting and uncommenting this

    // join allows the worker to finish because without it might
    // not even get cpu time
    // note that the thread will run anyway but, sometimes we might see the result of it
    // sometimes we might not

    // join returns a Result and for simplicity we just unwrap
    // without handling an error if present


    // Multiple Threads

    let h1: JoinHandle<()> = thread::spawn(|| {
        // thread::sleep(Duration::from_secs(1));
        println!("h1");
    });
    let h2: JoinHandle<()> = thread::spawn(|| {
        println!("h2");
    });
    h1.join().unwrap();
    h2.join().unwrap();

    // order is unknown
    // we could use sleep for demos

    // Ownership

    let numbers = vec![1, 2, 3];
    let h3 = thread::spawn(move || {
        println!("{:?}", numbers);
    });

    // we need move because the worker might outlive the current thread
    // so ownership is transferred to the worker
    // spawn() needs 'static because everything captured by the thread closure
    // needs to be alive as long as the thread is running
    // (ie we avoid dangling references)

    // Creating multiple threads
    let mut handles = Vec::new();
    for i in 0..5 {
        handles.push(
            thread::spawn(move || {
                println!("Thread {i}");
            })
        );
    }
    for h in handles {
        h.join().unwrap();
    }
}

fn arc_mutex_condvar() {
    // suppose we want multiple threads working on the same data
    // we might think of just using move in every closure
    // but this doesn't work because after the first move ownership is transferred
    // we might think of borrowing but that's not possible because of the 'static requirement
    // the solution to this problem is using Arc (Atomic Reference Counted)
    // to have multiple owners

    let numbers = Arc::new(vec![1,2,3]);
    let mut handles = Vec::new();
    for i in 0..3 {
        let numbers = Arc::clone(&numbers);
        handles.push(
            thread::spawn(move || {
                println!("Thread {}: {:?}", i, numbers);
            })
        )
    }
    for h in handles {
        h.join().unwrap();
    }

    // every thread points to the same vector
    // notice we do not modify the vector we just read it
    // because Arc only provides shared ownership
    // not mutable access

    // Arc<T> implements Send and Sync when T is thread safe
    // for example Arc<Vec<i32>> works but Arc<RefCell<i32>> doesn't
    // that's also why when we need to write as well we use
    // Arc<Mutex<T>>

    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(
            thread::spawn(move || {
                let mut value = counter.lock().unwrap();
                *value += 1;
            })
        )
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());

    // in this example counter is protected by mutex to provide mutual exclusive access
    // to each thread and this mutex is wrapped in Arc to let each thread own it

    // counter is the same variable in each thread and, we are able to write on it
    // in a determinist way with different threads preventing a race condition

    // when a thread calls counter.lock().unwrap()
    // it is trying to acquire the lock to have exclusive access to the variable

    // it returns a Result with Ok type MutexGuard<i32> not just i32 because of RAII
    // as long as the guard exists the mutex remains locked

    // therefore we unwrap ignoring an error in acquiring the lock
    // and we use * to get the actual value because MutexGuard<T> implements
    // the traits Deref and DerefMut

    // this is really powerful because we never call something like "release_lock()" or "unlock()"
    // because when guard exits scope it is dropped thanks to RAII and lock is
    // automatically released

    // the error case in acquiring the lock is not because we are not able to get it
    // because the current thread just waits it out
    // it is called Mutex Poisoning

    /*
    // let mut guard = counter.lock().unwrap();
    // *guard += 10;
    // panic!("Crash");
    */

    // imagine the data was halfway through an update and the program panics
    // rust cannot know in what state the variable is so it counter.lock()
    // returns Err(PoisonError) but it not something we need to worry about we just unwrap()

    // suppose now a thread should wait UNTIL something happens
    // we can use a condition variable to make it wait without using cpu
    // (avoiding busy waiting)

    let pair = Arc::new((
            Mutex::new(false),
            Condvar::new(),
        ));

    let worker_pair = Arc::clone(&pair);

    let handle = thread::spawn(move || {
        let (lock, cvar) = &*worker_pair;
        // & instead of *worker_pair.1
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!("Worker starts");
    });
    thread::sleep(Duration::from_secs(2));
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();
    handle.join().unwrap();

    // in this example
    // the idea is a tuple of mutex that protects data
    // and a Condvar wrapped in Arc
    // the boolean value in the mutex is our condition that we want to be verified
    // before the worker starts to do something

    // let (lock, cvar) = &*worker_pair
    // dereferences * the Arc first to get the tuple and then uses destructuring with &
    // to get references to the single members of the tuple elegantly

    // while !*started {
    //     started = cvar.wait(started).unwrap();
    // }

    // this is the crucial part
    // cvar.wait(started) waits for started to be true
    // and it does so without wasting cpu time
    // we use while to avoid spurius wakeups (value can be changed randomly)

    // in the main thread
    // wait 2 seconds and then acquire the lock in the same way and
    // call notify_one (or notify_all) to wake up the worker

    // Send and Sync traits
    // they are marker traits ie they have no methods

    // a type is Send if ownership of a value can be transferred safely to another thread
    // everything captured by the closure of the new thread must be Send

    // a type is Sync if a shared reference &T can be used safely from different threads
    // notice that this only works in case we are just reading the value
    // so we can avoid using a mutex in this case
}

fn channel() {

}