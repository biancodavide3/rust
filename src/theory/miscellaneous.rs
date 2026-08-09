pub fn main() {
    syntax();
    // macros();
    // raii();
}

fn syntax() {
    // :: means go inside this type/module
    let num = f64::sqrt(25.0);
    println!("{}", num);
    // String::new()
    // Vec::new()
    // Option::Some(5)
    // Result::Ok(2.0)

    // generic syntax
    // shorthand with just <> or longer with where
    // fn foo<T: Clone + Debug + Display>(...)
    /*
    fn foo<T>(...)
    where T: Clone + Debug + Display,
    {
    ...
    }
     */

    // impl keyword
    // 1) impl methods for a struct
    // impl Point { ... }
    // 2) impl a trait for a struct
    // impl Debug for Point { ... }
    // 3) generic syntax
    // impl<T> Container<T> { ... }
    // with traits
    // impl<T: Debug> Container<T> { ... }
    // combining
    // impl<T: Debug> Debug for Container<T> { ... }
    // impl as a type parameter (like interface java)
    fn print(value: impl std::fmt::Display) {   // anything that implements display
        println!("{value}");
    }
    // which is same as
    // fn greet<T: Display>(name: T) {
    //     println!("{name}");
    // }

    // Self keyword
    // inside an impl block it's that type
    //Point {
    //     x,
    //     y,
    // }
    // you can write
    // Self {
    //     x,
    //     y,
    // } like in the new function used as a constructor
    // also appears in functions
    // Animal {
    //    fn clone(&self) -> Self;
    //}
    // dyn means use dynamic dispatch
    // Box<dyn Animal>
    // can contain any Animal compiler doesn't know
    // Box<T: Animal>

    // & &mut
    // borrow

    // * dereference

    // -> return type

    // => only appears in match cases

    // _
    // either ignore variable
    // match something {
    //  ...
    //   _ => {}
    // }
    // let _ = calculate();
    // or type inference
    // let numbers: Vec<_> = ...

    // 'a 'b ... lifetimes

    // '_ anonymous lifetime as in
    // Formatter<'_>

    // ? operator with Result

    // as
    // for casting
    // let x = 5u32;
    // let y = x as i64;

    // .. range
    // as in
    // 0..10 goes from 0 included to 10 excluded

    //::<..> specify the generic type explicitly because sometimes the compiler
    // cannot infer
    let x = "42".parse::<i32>();
    // collect::<Vec<_>>()
}

fn macros() {
    // a macro is code that generates code at compile time
    // they accept different syntax compared to normal functions
    // and variable number of parameters
    // macro!() for function like
    // macro![] for collection like
    // macro!{} for block like
    println!("hello");
    let zeros: Vec<_> = vec![0; 3]; // [0, 0, 0]
    println!("{:?}", zeros);
    let formatted: String = format!("{}", "hello format");
    println!("{:?}", formatted);
    // panic!("error") // stops the program immediately
    assert!(5 > 2); // checks condition
    let result = 3 + 2;
    assert_eq!(result, 5);
    assert_ne!(5, 6);
    // todo!() // placeholder
    use std::fmt::Write;
    let mut output = String::new();
    write!(&mut output, "Hello {}", "David");   // useful for building strings
    println!("{}", output);
    // compile configuration checks
    if cfg!(debug_assertions) {
        println!("Debug build");
    }
    // #[derive(Debug, ...)] to derive traits in structs
    // procedural macros
    // #[tokio::main]
    // async fn main() {}
    // #[serde::Serialize]
    // struct User {}
}

struct A;
struct B;
impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping A");
    }
}
impl Drop for B {
    fn drop(&mut self) {
        println!("Dropping B");
    }
}

fn raii() {
    // resource acquisition is initialization
    // the idea is
    // when an object is created, it acquires a resource and when it is destroyed
    // it automatically releases that resource
    let s = "Hello".to_string(); // s acquires a resource in heap (String "Hello")
    println!("{}", s);
    // after s is not used anymore it is automatically released
    // you never do it explicitly
    // this can be manipulated with scope
    println!("Start");
    {
        let s = String::from("Hello");
        println!("{s}");
    }
    println!("End");
    // every type can implement the Drop trait and automatically call drop()
    // when ownership ends
    // variables are dropped in reverse order of creation
    {
        let a = A;
        let b = B;
    }
    // output is
    // Dropping B
    // Dropping A
    // RAII is not limited to heap memory
    // but can include anything from files, sockets, mutex locks, db connections and so on
}