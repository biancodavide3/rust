use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx1) = mpsc::channel::<u32>();
    let (tx2, rx2) = mpsc::channel::<u32>();

    let mut producers = Vec::new();
    for id in 0u32..2 {
        let tx1c = tx1.clone();
        let p = thread::spawn(move || {
            for k in 1..=2 {
                tx1c.send(id + k).unwrap();
            }
        });
        producers.push(p);
    }

    let transformer = thread::spawn(move || {
        for v in rx1 {
            tx2.send(v * v).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        let mut somma = 0u32;
        for v in rx2 {
            somma += v;
        }
        somma
    });

    for p in producers {
        p.join().unwrap();
    }

    transformer.join().unwrap();
    let somma = consumer.join().unwrap();

    println!("Somma dei quadrati: {}", somma);
}

// Domande
/*
1. il programma compila? se no, si giustifichi la risposta
2. si descriva il comportamento del programma spiegando il suo comportamento errato
3. si corregga il programma per ottenere la corretta esecuzione del programma che lo porti alla terminazione
        con la stampa del valore acquisito a valle della terminazione del thread consumer
4. si indichi il valore finale stampato dal programma
 */

// Risposte
/*
1. il programma compila
2. il programma utilizza due thread producer per produrre dei valori (in particolare 1, 2, 2 e 3)
e usa la variable tx1c come clone di tx1 ma si dimentica di fare il drop(tx1) e quindi for v in rx1
e' convinto che ci sia un altro trasmettitore e non chiude il canale e quindi il transformer non termina
e il programma rimane bloccato a transformer.join().unwrap()
3. aggiungere drop(tx1) tra la riga 15 e 16
4. 1 + 4 + 4 + 9 = 18

 */