pub fn main() {
    // a closure is an anonymous function (like lambda)
    // that can capture values from the surrounding space
    let add_one = |a: i32| a + 1;
    println!("{}", add_one(5));
    let add = |a: i32, b: i32| a + b;
    println!("{}", add(3, 4));
    let factor: i32 = 5;
    let multiply = |a: i32| a * factor; // this is not possible with
    // a normal function
    println!("{}", multiply(3));
    // 3 ways to capture described by implementing different traits
    // 1) capture by immutable borrow (nothing is moved) trait Fn
    let message: String = String::from("hello");
    let print = || {
        println!("{}", message);
    };
    print();
    println!("{}", message); // still works
    // 2) capture by mutable borrow trait FnMut
    // (nobody else may modify the variable while it's borrowed)
    let mut count: usize = 0;
    let mut increment = || {
        count += 1;
    };
    increment();
    increment();
    println!("{}", count);
    // 3) capture by move trait FnOnce
    // (the closure takes ownership)
    let text = String::from("text");
    let consume = || {
        drop(text);
    };
    consume();
    // println!("{}", text); doesn't work
    // move keyword, forcing transfer of ownership
    let text = String::from("hello");
    std::thread::spawn(move || {
        println!("{text}");
    });
    // println!("{}", text); doesn't work
    // useful with threads as that thread may outlive the current function

    // every Fn is also FnMut and FnOnce
    // every FnMut is also FnOnce
    // not every FnOnce is FnMut
    // so if you required FnOnce in function you accept any closure
    // if you require Fn a closure that consumes ownership will not be accepted

    // closures cannot be generic type of parameters are inferred
    // once or specified explicitly if you need generics use normal functions

    // passing a closure to a function
    println!("{}", apply(3, |x| x * 2));

    // iterator
    /*
    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }
     */
    // functional operations are lazy and performed only when required
    let numbers: Vec<_> = vec![1, 2, 3, 4];
    let doubled: Vec<_> = numbers
        .iter()
        .map(|x| x * 2)
        .collect(); // only here is the operation performed
    println!("{:?}", doubled);
    let filtered_doubled: Vec<_> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("{:?}", filtered_doubled);
    numbers
        .iter()
        .for_each(|x| {
            println!("{}", x);
        });
    let found = numbers
        .iter()
        .find(|x| **x > 3);
    println!("{}", found.unwrap()); // returns the first found
    let exists =
        numbers
            .iter()
            .any(|x| *x == 4);
    println!("{}", exists);
    let all_satisfy = numbers
        .iter()
        .all(|x| *x > 0);
    println!("{}", all_satisfy);
    let sum =
        numbers
            .iter()
            .fold(0, |acc, x| acc + x);
    println!("{}", sum);

    // iter, iter_mut,  into_iter
    // iter() lets you borrow, nothing moves
    for x in numbers.iter() {
        println!("{x}");
    }
    println!("{:?}", numbers); // still usable
    // iter_mut allows you to modify values
    let mut numbers_mut = vec![1, 2, 3];
    numbers_mut
        .iter_mut()
        .for_each(|x| *x *= 2);
    println!("{:?}", numbers_mut);
    // into_iter comsumes the collection
    let numbers_consumed = vec![1,2,3];
    for x in numbers_consumed.into_iter() {
        println!("{x}");
    }
    // println!("{:?}", numbers_consumed); doesn't work

    // real world example
    let products = vec![
        Product {
            name: "Laptop".into(),
            price: 1200.0,
            stock: 3,
        },
        Product {
            name: "Mouse".into(),
            price: 25.0,
            stock: 0,
        },
        Product {
            name: "Keyboard".into(),
            price: 90.0,
            stock: 10,
        },
    ];

    // products in stock
    let available: Vec<_> =
        products
            .iter()
            .filter(|p| p.stock > 0)
            .collect();

    // only names
    let names: Vec<_> =
        products
            .iter()
            .map(|p| p.name.clone())
            .collect();

    // total inventory value
    let total = products
        .iter()
        .map(|p| p.price * p.stock as f64)
        .sum::<f64>();

    println!("{:?}", available);
    println!("{:?}", names);
    println!("{:?}", total);
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
    stock: u32,
}

fn apply<F>(val: i32, op: F) -> i32
where F: Fn(i32) -> i32,    // using the Fn trait as this one does not need to modify
{
    op(val)
}

