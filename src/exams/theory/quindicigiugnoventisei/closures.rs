use std::sync::Arc;
use std::thread;

fn main() {
    let x = 10;
    
    let c1 = || println!("{}", x);
    
    let mut y = 0;
    let mut c2 = || {
        y += 1;
        println!("{}", y);
    };
    
    let s = String::from("hello");
    let c3 = || {
        drop(s);
    };
}

// Domande
// 1. quale trait implementa c1? si motivi la risposta
// 2. quale trait implementa c2? si motivi la risposta
// 3. quale trait implementa c3? si motivi la risposta

// Risposte
// 1. c1 implementa Fn perche prende un riferimento esclusivo immutabile alla variable x 
// per leggerne il contenuto
// 2. c2 implementa FnMut perche prende un riferimento mutabile di y e ne modifica il valore
// 3. c3 implementa FnOnce in quanto si tratta di una closure che puo essere esegutiva una sola volta perche' 
// esegue il drop di s

// seconda parte
/*
fn main2() {
    let message = String::from("Hello");
    let handle = thread::spawn(|| {
        println!("T2: {}", message);
    });
    println!("T1: {}", message);
}
 */

// Domande
// 1. si spieghino le problematiche relative al suddetto codice
// 2. correggere il codice per permettere di eseguire correttamente le due stampe

// Risposte

// 1. il codice ha 2 problemi: il primo consiste nel fatto che la closure 
// del thread figlio prende solamente in prestito il valore di message. la lifetime del messaggio non 
// dura abbastanza per il thread e quindi dobbiamo usare la keyword move;
// il secondo problema consiste nel fatto che ci servono due proprietari
// di message (cioe' il thread main e il figli) quindi dobbiamo usare un Arc

// 2. codice corretto
pub fn main2() {
    let message = Arc::new(String::from("Hello"));
    let message2 = Arc::clone(&message);
    let handle = thread::spawn(move || {
        println!("T2: {}", *message2);    
    });
    println!("T1: {}", *message);
    handle.join().unwrap();
}