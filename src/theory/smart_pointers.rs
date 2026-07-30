use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};

trait Animal {}
struct Dog {}
struct Cat {}
impl Animal for Dog {}
impl Animal for Cat {}

enum List {
    Node(i32, Box<List>),
    End,
}

pub fn main() {
    // Box
    let x: Box<i32> = Box::new(5);
    // memory is on the heap instead of the stack and is freed automatically
    println!("{}", *x); // dereferentiation because it implements the Deref trait
    // useful for large structures or recursion because they need large amounts of memory
    // let data = Box::new(vec![0, 1_000_000]);
    // or for dynamic dispatch
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog{}),
        Box::new(Cat{}),
    ];

    // Rc (Reference Counted)
    // it is used to have multiple owners
    let a: Rc<String> = Rc::new(String::from("hello"));
    let b: Rc<String> = Rc::clone(&a);
    let c: Rc<String> = Rc::clone(&a);
    // it is dropped only when the count of strong references is 0
    println!("{}", Rc::strong_count(&a));   // 3
    drop(b);
    println!("{}", Rc::strong_count(&a));   // 2
    // there is also weak references
    let d: Weak<String> = Rc::downgrade(&a);
    println!("{}", Rc::strong_count(&a));   // still 2
    println!("{}", Rc::weak_count(&a));   // 1
    // this is so that it doesn't keep the memory alive if it doesn't have to
    // need to upgrade it before using because it might return None
    match d.upgrade() {
        Some(val) => println!("{}", val),
        None => println!("None"),
    }
    // assume now strong count == 0
    drop(a);
    drop(c);
    match d.upgrade() {
        Some(val) => println!("{}", val),
        None => println!("None"),
    }

    // Arc (Atomic Reference Counted)
    // it is the same as Rc but, it's thread safe

    // RefCell
    // moves checking borrowing rules at runtime instead of compile time
    let refcell: RefCell<String> = RefCell::new(String::from("hello world"));
    let immut_borrow = refcell.borrow();
    println!("{}", immut_borrow);
    // let mut_borrow = refcell.borrow_mut(); program could compile but, it would panic at runtime
    // use it for interior mutability imagine Rc<RefCell<...>>
}