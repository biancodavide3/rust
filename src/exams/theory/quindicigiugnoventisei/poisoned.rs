use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let data2 = Arc::clone(&data);

    let handle = thread::spawn(move || {
        let mut value = data2.lock().unwrap();
        *value = 10;
        panic!("errore nel thread");
    });

    let _ = handle.join();

    // Domanda 3: si fa riferimento a questo unwrap();
    // Domanda 4: si sostituisca questa linea di codice
    let value = data.lock().unwrap();

    println!("{}", *value);
}

// Domande
/*
1. il codice compila?
2. che cosa succede a runtime?
3. perche la chiamata unwrap() sul secondo lock() (dopo i commenti) puo' causare un panic?
4. Come si puo evitare questo panic? si scriva un frammento di codice opportuno (si veda il commento)
 */

// Risposte
/*
1. il codice compila correttamente
2. il thread figlio modifica il valore protetto dal mutex e poi panica mentre e' ancora in possesso del lock
e quindi il mutex diventa poisoned
nel thread principale
let _ = handle.join() restituisce un errore ma viene ignorato
let value = data.lock().unwrap(); causa un panic
3. la chiamata data.lock().unwrap() causa un panic perche' lock() restituisce Err(PoisonError) e quindi unwrap panica
(invece di avere un Ok(MutexGuard<...>) e quindi unwrap ci ritornerebbe MutexGuard<...>)
4. per evitare questo panic possiamo fare un match esplicito su data.lock()
match data.lock() {
Ok(value) => println!("{}", *value),
Err(poisoned) => {
let value = poisoned.into_inner(); // recupero comunque il valore anche se poisoned
// ... cerco di recuperare il programma
println("{}", *value);
}
}
 */