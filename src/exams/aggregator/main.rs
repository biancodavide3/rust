use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Average {
    pub sensor_id: usize,
    pub reference_time: Instant,
    pub average_temperature: f64,
}

struct State {
    // sensor_id -> (sum, number of measurements)
    measures: HashMap<usize, (f64, usize)>,
    // sensor_id -> Average
    averages: HashMap<usize, Average>,
    shutdown: bool,
}

pub struct Aggregator {
    shared: Arc<(Mutex<State>, Condvar)>,
    worker: Option<thread::JoinHandle<()>>,
}

impl Aggregator {
    pub fn new(sample_time_millis: u64) -> Self {
        let shared = Arc::new((
            Mutex::new(State {
                measures: HashMap::new(),
                averages: HashMap::new(),
                shutdown: false,
            }),
            Condvar::new()
        ));
        let worker_shared = Arc::clone(&shared);
        let worker = thread::spawn(move || {
            let (mutex, condvar) = &*worker_shared;
            let period = Duration::from_millis(sample_time_millis);
            loop {
                let mut state = mutex.lock().unwrap();
                state = condvar.wait_timeout(state, period).unwrap().0;
                if state.shutdown {
                    break;
                }
                let mut averages = HashMap::new();
                let reference_time = Instant::now();
                for (&sensor_id, &(sum, count)) in &state.measures {
                    averages.insert(
                        sensor_id,
                        Average {
                            sensor_id,
                            reference_time,
                            average_temperature: sum / count as f64,
                        },
                    );
                }
                state.averages = averages;
                state.measures.clear();
            }
        });
        Self {
            shared,
            worker: Some(worker),
        }
    }

    pub fn add_measure(&self, sensor_id: usize, temperature: f64) {
        // aggiunge una misura di temperatura per il sensore con id `sensor_id` // e temperatura `temperature`. Le misure sono automaticamente etichettate
        // con l'istante temporale in cui sono comunicate.
        let (mutex, _) = &*self.shared;
        let mut state = mutex.lock().unwrap();
        let entry = state.measures
            .entry(sensor_id)
            .or_insert((0.0, 0));
        entry.0 += temperature;
        entry.1 += 1;
    }

    pub fn get_averages(&self) -> Vec<Average> {
        // restituisce un vettore che riporta la temperatura media di ciascun sensore,
        // calcolata durante l'ultimo periodo di campionamento.
        // Sono presenti solo i sensori che hanno inviato almeno una misura.
        let (mutex, _) = &*self.shared;
        let state = mutex.lock().unwrap();
        state.averages.values()
            .map(|average| Average {
                sensor_id: average.sensor_id,
                reference_time: average.reference_time,
                average_temperature: average.average_temperature,
            })
            .collect()
    }
}

impl Drop for Aggregator {
    fn drop(&mut self) {
        let (mutex, condvar) = &*self.shared;
        {
            let mut state = mutex.lock().unwrap();
            state.shutdown = true;
            condvar.notify_one();
        }
        if let Some(worker) = self.worker.take() {
            worker.join().unwrap();
        }
    }
}

