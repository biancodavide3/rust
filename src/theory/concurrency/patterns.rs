use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn main() {
    // 1. fire and forget worker
    //fire_forget();

    // when main thread doesn't care about result from the worker
    // main -> spawn thread -> continues immediately
    // worker -> does work -> dies

    // skeleton
    /*
    thread::spawn(|| {
        do_work();
    });
     */

    // 2. fork join
    //fork_join();

    // split work that does not need synchronization
    // between different independent worker
    // finally combine the result

    // image -> split -> thread 1, 2, 3 ... -> join()

    // skeleton
    /*
    let mut handles = Vec::new();
    for chunk in work {
        handles.push(thread::spawn(move || {
            process(chunk)
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
     */

    // 3. shared state (with or without waiting a condition)
    // shared_state();
    // waiting_for_condition();

    // threads must share some state to solve a problem

    // arc -> mutex -> condvar

    // skeleton
    /*
    let data = Arc::new(Mutex::new(...));
    let clone = Arc::clone(&data);
    thread::spawn(move || {
        let mut value = clone.lock().unwrap();
        ...
    });
     */

    // 4. producer consumer
    // producer_consumer();

    // producer sends work to consumer

    // producer -> channel -> consumer

    // skeleton
    /*
    let (tx, rx) = channel();

    thread::spawn(move || {

        tx.send(job).unwrap();

    });

    let job = rx.recv().unwrap();
     */

    // 5. multiple producers
    // multiple_producers();

    // still supported by standard rust mpsc
    // just add

    // tx2 = tx.clone()

    // and send more work from it to consumer

    // 6. dispatcher / fan out
    // dispatcher();

    // one thread creates work
    // many thread consume it

    // it is not supported by mpsc
    // but, we have a workaround by wrapping the rx in an Arc

    // 7. fan in
    // fan_in();

    // many workers produce one final result
    // usually each thread returns with join() or sends result with a channel

    // skeleton
    /*
    let h = thread::spawn(|| {
        compute()
    });
    let result = h.join().unwrap();
    */
    // or
    // tx.send(result)

    // 8. request response
    // request_response();

    // looks like networking
    // main -> request -> worker -> response -> main

    // usually 2 channels
    // tx_request -> worker -> tx_response


    // 9. pipeline
    // pipeline();

    // work goes through different stages each one with its own thread
    // and they communicate using channels

    // it scales pretty well

    // Reader -> channel -> Parser -> channel -> Processor -> channel -> Writer
}

fn fire_forget() {
    thread::spawn(|| {
        println!("Saving logs...");
        thread::sleep(Duration::from_secs(2));
        println!("Logs Saved!");
    });
    println!("Main continues immediately");
}

fn fork_join() {
    let h1 = thread::spawn(|| {
        let chunk = (1..500);
        return chunk.sum::<i32>();
    });
    let h2 = thread::spawn(|| {
        return (500..1000).sum::<i32>();
    });
    let total = h1.join().unwrap() + h2.join().unwrap();
    println!("{}", total);
}

fn shared_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(
            thread::spawn(move || {
                let mut value = counter.lock().unwrap();
                *value += 1;
            })
        );
    }

    for h in handles {
        h.join().unwrap();
    }

    let value = counter.lock().unwrap();
    println!("{}", value);
}

fn producer_consumer() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
    });

    for val in rx {
        println!("received {}", val);
    }
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    for i in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("Sensor {}", i)).unwrap();
        });
    }

    drop(tx);

    for msg in rx {
        println!("{}", msg);
    }
}

fn dispatcher() {
    let (tx, rx) = mpsc::channel::<i32>();

    let rx = Arc::new(Mutex::new(rx));
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        thread::spawn(move || {
            loop {
                let value = rx.lock().unwrap().recv();
                match value {
                    Ok(job) => println!("Worker {} processed job {}", id, job),
                    Err(_) => break,
                }
            }
        });
    }

    for job in 0..10 {
        tx.send(job).unwrap();
    }

    drop(tx);
    thread::sleep(Duration::from_secs(1));
}

fn fan_in() {
    let (tx, rx) = mpsc::channel::<i32>();

    for i in 1..=5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i * i).unwrap();
        });
    }

    drop(tx);

    let total = rx.iter().sum::<i32>();
    println!("{}", total);
}

fn request_response() {
    let (request_tx, request_rx) = mpsc::channel();
    let (response_tx, response_rx) = mpsc::channel();

    thread::spawn(move || {
        while let Ok(n) = request_rx.recv() {
            response_tx.send(n + 1).unwrap();
        }
    });

    request_tx.send(5).unwrap();
    request_tx.send(7).unwrap();
    request_tx.send(11).unwrap();

    drop(request_tx);

    for result in response_rx {
        println!("result {}", result)
    }
}

fn waiting_for_condition() {
    let shared = Arc::new((
        Mutex::new(false),
        Condvar::new(),
    ));

    let worker = Arc::clone(&shared);

    thread::spawn(move || {
        let (lock, cvar) = &*worker;
        let mut ready = lock.lock().unwrap();
        while !*ready {
            ready = cvar.wait(ready).unwrap();
        }
        println!("Configuration loaded!");
    });

    thread::sleep(Duration::from_secs(1));
    let (lock, cvar) = &*shared;
    let mut ready = lock.lock().unwrap();
    *ready = true;
    cvar.notify_one();
}

fn pipeline() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        tx1.send("hello".to_string()).unwrap();
    });
    thread::spawn(move || {
        while let Ok(res) = rx1.recv() {
            tx2.send(res.to_uppercase()).unwrap();
        }
    });
    while let Ok(res) = rx2.recv() {
        println!("final result = {}", res);
    }
}

