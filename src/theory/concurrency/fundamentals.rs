use std::thread;

pub fn main() {
    // concurrency is when a single cpu core switches rapidly between different tasks
    // executing them during the same period of time

    // true parallelism is when multiple cores execute different tasks at the same

    // programs are executed in the form of processes that are loaded in memory
    // from the disk and provide the cpu with instructions in its assembler language

    // a thread is an independent execution path within a process

    let handle = thread::spawn(|| {
        println!("Hello from child");
    });
    handle.join().unwrap();

    // threads are useful because they can help with performance
    // like in a long calculation that can be parallelized
    // or for responsiveness in gui processing
    // or background work in general

    // there are 4 different classic problems that occur when threads interact with each other
    // and these are the reason why concurrency is hard

    // 1) Race Conditions
    // suppose counter += 1
    // thread A reads 5
    // thread B reads 5
    // thread A writes 6
    // thread B writes 6
    // expected result is 7
    // but actual is 6
    // the problems lies in timing (ie who wins the race)

    // 2) Data Races
    // a specific race conditions that involves unsynchronized memory access
    // suppose
    // thread A writing variable
    // thread B reading variable
    // result is unpredictable

    // 3) Deadlock
    // when no one can make progress because everyone is waiting
    // suppose
    // thread A locks Mutex A and waits for Mutex B
    // thread B locks Mutex B and waits for Mutex A
    // no thread can make progress and they are stuck forever

    // 4) Starvation
    // it happens when a thread can never get the lock (ie it is starved)
    // because another one always get it
    // this way the starved thread can never make progress

    // There are 2 major ways thread communicate

    // 1) shared memory
    // all threads share the same object in memory
    // it needs synchronization
    // with primitives such as Mutex, Atomics, RwLock

    // 2) message passing
    // threads do not share data
    // they send each other messages
    // in rust we use channels

    // Synchronizing threads means to coordinate them
    // so that they don't interfere with each other

    // Critical Section
    // it is a piece of code that accesses shared data and therefore must not
    // be executed by multiple threads at the same time
    // let mut value = counter.lock().unwrap();
    // *value += 1; // critical section

    // Mutual Exclusion
    // if one thread is inside, everyone else waits

    // Thread Safety
    // a type is thread safe it can be used correctly by different thread without
    // causing undefined behavior
    // in rust we have 2 important marker traits Send and Sync

}