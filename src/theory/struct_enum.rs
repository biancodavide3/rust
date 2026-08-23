use std::fmt::{Display, Formatter};

pub fn main() {
    println!("BEING ENUM EXAMPLE -------------");
    enum_example();
    println!("BEING STRUCT EXAMPLE -----------");
    struct_example();
}

enum Direction {
    North,
    East,
    South,
    West
}

enum Message {
    Quit,
    Move(i32, i32),
    Write(String)
}

fn enum_example() {
    let direction = Direction::North;
    // enum values can also store data
    let message = Message::Move(12, 11);
    // pattern matching
    match &message {    // use reference to prevent move of the value
        Message::Quit => {
            println!("quitting");
        }
        Message::Move(x, y) => { // destructuring
            println!("{x} {y}");
        }
        Message::Write(s) => {
            println!("{s}");
        }
    }
    // ignoring the other values
    match &message {
        Message::Move(x, y) => {
            println!("{} {}", x, y);
        }
        _ => {}
    }
    // or with if let
    if let Message::Move(x, y) = &message {
        println!("{} {}", x, y);
    }

    // option like optional in java
    /*
    enum Option<T> {
        None,
        Some(T),
    }
     */

    let x = Some(5);
    match x {
        Some(value) => println!("{value}"),
        None => println!("none"),
    }

    // with if let to just get ignore None

    if let Some(value) = x {
        println!("{value}");
    }

    let x2: Option<i32> = None;

    // useful methods
    println!("{}", x.unwrap()); // panics if None
    println!("{}", x.expect("Expected a number")); // better error handling
    println!("{}", x2.unwrap_or(6)); // if None you get default
    println!("{}", x2.unwrap_or_else(|| {
        println!("Computing default...");
        15
    }));
    let mut x3 = Some(20);
    let x4 = x3.take();
    println!("x3 value was taken and now is: {:?}, and it is now in {:?}", x3, x4);

    // result with success and error type
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     */
    let success: Result<i32, String> = Ok(5);
    let error: Result<i32, String> = Err("error".to_string());
    match success {
        Ok(result) => println!("{}", result),
        Err(error) => println!("{}", error),
    }

    // ? operator
    // we propagated the error up to the main function now we can either unwrap and panic in case of error
    // or handle it with an explicit match
    let final_result = calculate().unwrap();
    println!("{}", final_result);
}

fn calculate() -> Result<i32, String> {
    // without ?
    /*
    let value = divide(10, 2);
    match value {
        Ok(result) => Ok(result),
        Err(error) => Err(error),
    }
     */
    // with ? you can easily to multiple operations
    let x = divide(10, 2)?;
    let y = divide(25, x)?;
    Ok(y)
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(a/b)
    }
}


struct User {
    username: String,
    email: String,
    age: i8
}

struct Rectangle {
    x: f64,
    y: f64
}

// implementing a method for a struct

impl Rectangle {
    // immutable reference to the struct with &self
    fn area(&self) -> f64 {
        self.x * self.y
    }

    // mutable reference
    fn grow(&mut self) -> () {
        self.x += 10.0;
    }

    // taking ownership
    fn destroy(self) -> () {
        println!("gone");
    }

    // if there is no self it could be used as a constructor (associated function)
    fn new(x: f64, y: f64) -> Rectangle {
        Rectangle {
            x, y
        }
    }
}

fn struct_example() {
    let name = "David".to_string();
    let mut user: User = User {
        username: name,
        email: "david@email.com".to_string(),
        age: 22
    };
    user.age = 23;  // struct must be mut for this
    // println!({}, name); this does not work as name is moved in the struct
    println!("{}", user.email);
    // shortcut to create struct is to have the variables as the same name as the fields of it
    let x: f64 = 10.0;
    let y: f64 = 2.5;
    let mut rec = Rectangle {x, y}; // as to be mut to call methods with &mut self within the struct
    println!("{}", rec.area());
    rec.grow();
    println!("{}", rec.area());
    rec.destroy();
    let rec = Rectangle::new(2.0, 7.0);
    println!("{}", rec.area());
}