struct Resource {
    name: String
}

impl Resource {
    fn new(name: &str) -> Resource {
        Resource {
            name: String::from(name),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("    => Dropping {}", self.name);
    }
}

fn consume_resource(r: Resource) {
    println!("Consuming {}", r.name);
}

fn borrow_resource(r: &Resource) {
    println!("Borrowing {}", r.name)
}

fn main() {
    let r1 = Resource::new("R1");

    {
        let r2 = Resource::new("R2");
        borrow_resource(&r2);

        let r3 = Resource::new("R3");
        consume_resource(r3);

        println!("End inner block");
    }

    borrow_resource(&r1);

    let r4 = Resource::new("R4");
    let _r5 = r4;

    println!("End Main");
}

// domande
/*
1. in quale ordine vengono stampati i messaggi
2. quando viene invocato automaticamente il metodo drop?
3. che cosa si intende per RAAi?
4. perche questo meccanismo e' importante nella programmazione di sistema
5. e' possibile chiamare esplicitamente r1.drop()? motivare la risposta
 */

// risposte
/*
1. borrowing r2 -> consuming r3 -> dropping r3 ->  end inner block -> dropping r2 ->
 borrowing r1 -> end main -> dropping r4 -> dropping r1

 borrow_resource(&r2) stampa borrowing r2
 consume_resource(r3) stampa consuming r3
 alla fine di consume_resource(r3) esce di scope quindi viene invocato drop r3
 stampa end inner block
 r2 esce di scope e quindi drop r2
 borrow r1
 r4 viene spostato in r5 e poi esce di scope -> drop r4
 infine distrutto r1

2. drop viene invocato automaticamente quando il proprietario del valore esce di scope. se il valore viene spostato
(move), il distruttore verra eseguito quando uscira di scope il nuovo proprietario. se il valore viene passato per valore a una
nuova funzione (come consume_resource) verra distrutto alla fine di quella funzione
3. RAII (Resource acquisition is initialization) e' un paradigma secondo cui una risorsa viene acquisita
durante la costruzione dell'oggetto e rilasciata automaticamente quando l'oggetto viene distrutto. in rust questo
avviene grazie all'ownership e il tratto Drop senza la necessita di rilasciare manualmente le risorse
4. RAII e' importante nella programmazione di sistema perche' garantisce il rilascio automatico di risorse (memoria, file
socket, mutex ecc.) anche in presenza di errori o ritorni anticipati. in questo modo si evitano problemi come memory leak o
doppie liberazioni
5. non e' possibile usare r1.drop() e' obbligatorio std::mem::drop(r1)
 */