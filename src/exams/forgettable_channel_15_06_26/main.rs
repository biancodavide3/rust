use std::sync::{Arc, Mutex, mpsc};

pub trait Forgettable {
    fn forget(&self) -> bool;
}

pub trait ForgettableReceiver<T: Send + 'static> {
    fn recv(&self) -> Option<T>;
}

pub trait ForgettableSender<T: Send + 'static>: Clone {
    fn send(&self, t: T) -> Option<impl Forgettable + 'static>;
}

struct Message<T> {
    value: Option<T>,
    forgotten: bool,
}

struct ForgetHandle<T> {
    shared: Arc<Mutex<Message<T>>>,
}

impl<T> Forgettable for ForgetHandle<T> {
    fn forget(&self) -> bool {
        let mut msg = self.shared.lock().unwrap();

        // il receiver l'ha già consumato
        if msg.value.is_none() {
            return false;
        }

        // idempotente
        msg.forgotten = true;
        true
    }
}

struct MySender<T: Send + 'static> {
    tx: mpsc::Sender<Arc<Mutex<Message<T>>>>,
}

struct MyReceiver<T: Send + 'static> {
    rx: Mutex<mpsc::Receiver<Arc<Mutex<Message<T>>>>>,
}

impl<T: Send + 'static> Clone for MySender<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone()
        }
    }
}

impl<T: Send + 'static> ForgettableSender<T> for MySender<T> {
    fn send(&self, t: T) -> Option<impl Forgettable + 'static> {

        let msg = Arc::new(Mutex::new(Message {
            value: Some(t),
            forgotten: false,
        }));

        if self.tx.send(msg.clone()).is_err() {
            return None;
        }

        Some(ForgetHandle { shared: msg })
    }
}

impl<T: Send + 'static> ForgettableReceiver<T> for MyReceiver<T> {

    fn recv(&self) -> Option<T> {

        loop {

            let shared = {
                let rx = self.rx.lock().unwrap();

                match rx.recv() {
                    Ok(v) => v,
                    Err(_) => return None,
                }
            };

            let mut msg = shared.lock().unwrap();

            if msg.forgotten {
                // messaggio annullato
                continue;
            }

            return msg.value.take();
        }
    }
}

pub fn forgettable_channel<T: Send + 'static>()
    -> (impl ForgettableSender<T>, impl ForgettableReceiver<T>)
{
    let (tx, rx) = mpsc::channel();

    (
        MySender { tx },
        MyReceiver {
            rx: Mutex::new(rx),
        },
    )
}