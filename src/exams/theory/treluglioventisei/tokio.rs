/*
use tokio::time::{sleep, Duration};
async fn task(nome: &str, secondi: u64) {
    println!("Inizio {}", nome);
    sleep(Duration::from_secs(secondi)).await;
    println!("Fine {}", nome);
}
#[tokio::main]
async fn main() {
    let f1 = task("A", 2); // Domanda B
    let f2 = task("B", 1);
    println!("Future create");
    f1.await;
    f2.await;
    println!("Main terminato");
}
 */

// domande
/*
A. perche e' necessario l'attributo #[tokio::main]
B. che cosa produce la chiamata task("A", 2) corrispondente all'istruzione commentata?
l'esecuzione del corpo inizia immediatamente?
C. in quale ordine vengono stampati i messaggi e qual e' il tempo totale approssimativo di esecuzione?
D. Come si potrebbe modificare il main per ridurre il tempo di esecuzione, mantenendo l'esecuzione dei due task?
si scriva il frammento di codice modificato e si indichi il nuovo tempo.
 */

// risposte
/*
A. si tratta di un programma asincrono che utlizza async await e tokyo e' il runtime necessario per utilizzare
queste feature in rust
B. non produce nulla immeditamente perche' si tratta di una future e per ottenere il risultato bisogna utilizzare .awai come
e' stato fatto sotto
C. future create -> inizio a -> (dopo circa 2 secondi) fine a -> inizio b -> (dopo circa 1 sec) fine b -> main terminato
e quindi il tempo totale e' circa 3 secondi
D. potremmo utilizzare una join invece di fare 2 await singoli in modo che l'esecuzione delle due task inizi in contemporanea
cosi il tempo di eseuzione passa a circa 2 secondi
tokyo::join!(f1, f2);
 */