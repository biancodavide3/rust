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
        _=> {}
    }
    // or with if let
    if let Message::Move(x, y) = &message {
        println!("{} {}", x, y);
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