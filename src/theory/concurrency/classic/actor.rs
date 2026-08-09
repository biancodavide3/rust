use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// use dining philosopher problem to show this approach
// we no longer think of a fork as a shared resource that philosopher acquire
// a fork is an independent object/actor that owns its own state ( available variable )
// that philosophers communicate with using channels

// so a fork runs its own thread waiting to receive messages on a loop
// if that fork is available granting to a philosopher that wants it
// otherwise put the request in a queue
// when releasing a fork serve the queue first otherwise
// set available = true to receive new requests

// a philosopher sends a take request to a fork with the specific enum value but also sends
// a sender part of a channel that the fork can you to tell the philosopher when it is available
// that works because the philosophers puts itself in a waiting with recv() for that same channel

// the fork internally manages its state setting available = false so new requests are put in queue
// when that philosopher has finished eating it sends a release request so the fork can update
// its state or serve the queue

// stop is just used to exit gracefully terminating the threads

// still using a strategy such as resource ordering in this problem to prevent deadlock

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

enum ForkMessage {
    Take(mpsc::Sender<()>),
    Release,
    Stop,
}

struct Fork {
    id: usize,
    receiver: mpsc::Receiver<ForkMessage>,
}

// Fork owns its state and communicates only through messages
impl Fork {
    fn run(self) {
        let mut available = true;
        let mut waiting = VecDeque::new();

        while let Ok(message) = self.receiver.recv() {
            match message {
                ForkMessage::Take(reply) => {
                    if available {
                        available = false;
                        reply.send(()).unwrap();

                        println!("Fork {} acquired", self.id);
                    } else {
                        // Fork is busy, remember the request.
                        waiting.push_back(reply);
                    }
                }

                ForkMessage::Release => {
                    if let Some(reply) = waiting.pop_front() {
                        // Immediately give the fork to
                        // the next waiting philosopher.
                        reply.send(()).unwrap();

                        println!(
                            "Fork {} passed to waiting philosopher",
                            self.id
                        );
                    } else {
                        available = true;

                        println!(
                            "Fork {} released",
                            self.id
                        );
                    }
                }

                ForkMessage::Stop => break,
            }
        }
    }
}

// Handle used by philosophers to communicate with a fork actor
#[derive(Clone)]
struct ForkHandle {
    sender: mpsc::Sender<ForkMessage>,
}

impl ForkHandle {
    fn take(&self) {
        let (reply_tx, reply_rx) = mpsc::channel();

        self.sender
            .send(ForkMessage::Take(reply_tx))
            .unwrap();

        // Wait until the fork actor grants ownership
        reply_rx.recv().unwrap();
    }

    fn release(&self) {
        self.sender
            .send(ForkMessage::Release)
            .unwrap();
    }

    fn stop(&self) {
        self.sender
            .send(ForkMessage::Stop)
            .unwrap();
    }
}

// -----------------------------
// Actor table
// -----------------------------

struct ActorTable {
    forks: Vec<ForkHandle>,
}

impl ActorTable {
    fn new(size: usize) -> Self {
        let mut forks = Vec::new();

        for id in 0..size {
            let (sender, receiver) = mpsc::channel();

            let fork = Fork { id, receiver };

            thread::spawn(move || {
                fork.run();
            });

            forks.push(ForkHandle { sender });
        }

        Self { forks }
    }

    fn run(&self, philosophers: Vec<Philosopher>) {
        let mut handles = Vec::new();
        let size = self.forks.len();

        for philosopher in philosophers {
            let left = self.forks[philosopher.id].clone();
            let right = self.forks[(philosopher.id + 1) % size].clone();

            handles.push(thread::spawn(move || {
                for _ in 0..3 {
                    philosopher.think();

                    // Deadlock prevention:
                    // always acquire the lower-numbered fork first.
                    let left_id = philosopher.id;
                    let right_id = (philosopher.id + 1) % size;

                    let (first, second) = if left_id < right_id {
                        (left.clone(), right.clone())
                    } else {
                        (right.clone(), left.clone())
                    };

                    first.take();
                    println!("{} acquired first fork", philosopher.name);

                    second.take();
                    println!("{} acquired second fork", philosopher.name);

                    philosopher.eat();

                    second.release();
                    first.release();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn stop(&self) {
        for fork in &self.forks {
            fork.stop();
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

    let table = ActorTable::new(5);

    table.run(philosophers);
    table.stop();
}