struct Parser<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Parser<'a> {
        Parser { text, pos: 0 }
    }
    fn rest(&self) -> &str {
        &self.text[self.pos..]
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("un testo abbastanza lungo");
    let risultato;
    {
        let s2 = String::from("breve");
        risultato = longest(&s1, &s2);
        println!("Più lungo: {}", risultato);
        // il lifetime del riferimento di risultato vive quindi fino a quello di s2
        // perche' &s2 vive molto meno e termina tra queste parentesi quindi non si puo'
        // usare dopo anche se dichiarato prima
    }

    let p = Parser::new(&s1);
    println!("Resto: {}", p.rest());
    // Domanda B: si consideri di decommentare la riga seguente
    // println!("Ancora: {}", risultato);
}

// domande
/*
1. Che cosa significa l'annotazione 'a nella funzione longest? Quale relazione stabilisce tra i parametri e il
valore di ritorno? (0,5pt)
2. Se si decommenta la riga indicata, il programma compila? Motivare la risposta. (1pt)
3. Nel metodo rest non compare alcuna annotazione esplicita sul tipo di ritorno: perché il codice è
comunque valido? (0,5pt)
4. Perché la struct Parser deve dichiarare il parametro <'a>? Cosa accadrebbe senza? (0,5pt)
5. Si potrebbe riscrivere longest con due lifetime distinti, fn longest<'a,
    'b>(x: &'a str, y: &'b str) ->
&'a str? In quali casi compilerebbe? (0,5pt)
 */

// 1.
/*
'a e' un parametro di lifetime che stabilisce una relazione tra i lifetime dei riferimenti
di x e y e del riferimento di ritorno. la relazione consiste nel fatto che il lifetime del
riferimento di ritorno e' limitato alla porzione di tempo in cui i riferimenti x e y sono entrambi
validi cioe all'intersezione dei loro lifetime
 */

// 2.
/*
il programma non compila. risultato contiene un riferimento che vive fino all'intersezione dei
lifetime di &s1 e &s2. poiche' s2 viene distrutta all'interno del blocco delimitato dalle parentesi
graffe anche risultato e' distrutto.
la funzione longest potrebbe ritornare proprio &s2 e questo produrrebbe una dangling reference,
che e' una possibilita che rust impedisce
 */

// 3.
/*
il codice e' comunque valido perche' il lifetime puo' essere omesso grazie alle lifetime elision rules
di rust. in particolare per un metodo che riceve &self il lifetime del riferimento restituito
viene automaticamente associato al lifetime di self
esempio
fn rest<'a>(&'a self) -> &'a str { ... }
 */

// 4.
/*
la struct Parser deve dichiarare il parametro <'a> perche' contiene il riferimento text: &'a str
questo garantisce che un Parser non possa vivere piu' a lungo della stringa a cui fa riferimento
ed evita quindi il rischio di dangling reference
 */

// 5.
/*
si potrebbe riscrivere la funzione con due lifetime distinti fn longest<a, b>(&'a str x, &'b str y)
-> &'a str ma in questo caso dovremmo essere sicuri che la funzione ritorni x perche' e' cio' che e'
stato promesso dalla firma della funzione
 */

// riassunto
/*
i lifetime, indicati esplicitamente con 'a 'b 'c ... , specificano la durata per la quale
un riferimento e' valido. il loro scopo principale e' quello di evitare le dangling references
che sono proibite da rust. nella maggior parte dei casi il compilatore e' in grado di
dedurre i lifetime dei riferimenti coinvolti grazie alle lifetime elision rules
nel caso di riferimenti locali
fn main() {
    let s = String::from("hello");
    let r = &s;
    println!("{}", r);
}
o nel metodo di una struct che ritorna un riferimento e ha come parametro &self
in questi casi i riferimenti hanno lo stesso lifetime
in alcuni casi deve essere esplicitato con la notazione 'a
come nel caso di una struct che contiene un riferimento
struct Parser<'a> {
    text: &'a str,
    pos: usize,
}
nota se usassimo String allora la struct possederebbe text e quindi non dovremmo esplicitare
o nel caso di funzioni che hanno piu parametri in input
esempio
il riferimento di ritorno ha la stessa durata dei due riferimenti in input quindi vive fino all'intersezione
della durata dei due riferimenti
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
se cambiassiamo y:'b str allora dovremmo essere sicuri che la funzione ritorni sempre x
perche' guardiamo a cosa promette la firma della funzione prima del corpo
 */