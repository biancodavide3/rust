use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

pub fn main() {
    let dog = Dog {
        name: "MyDog".to_string()
    };

    let cat = Cat {
        name: "MyCat".to_string()
    };

    make_sound(&dog);
    make_sound_generics(&cat);

    let dog_clone = dog.clone();
    println!("{:?}", dog_clone);

    let config = Config::default();
    println!("{:?}", config);

    let p0 = Point::new(0, 0);
    let p1 = Point::new(1, 3);
    let p2 = Point::new(1, 3);
    let p3 = Point::new(1, 4);

    println!("{} {}", p1 == p2, p2 != p3);

    println!("{:?}", p1 + p2);

    let d10 = Distance::new(p1, p0);
    let d20 = Distance::new(p2, p0);
    let d32 = Distance::new(p3, p2);

    println!("{} {} {}", d10 > d20, d10 == d20, d32 > d10);

    let file = File;

    let number = MyNumber::from(5);
    println!("{:?}", number);

    // trait object: dynamic dispatch (kind of like using an interface as a type in java)
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {name : "myDog".to_string()}),
        Box::new(Cat {name: "myCat".to_string()}),
    ];

    for animal in animals {
        animal.speak();
    }
}

// using traits as an interface or contract

trait Animal {
    fn speak(&self) -> () {
        println!("default impl");
    }
}

#[derive(Debug, Clone)]
struct Dog {
    name: String
}

struct Cat {
    name: String
}

impl Animal for Dog {
    fn speak(&self) -> () {
        println!("Woof {}", self.name);
    }
}

impl Animal for Cat {
    fn speak(&self) -> () {
        println!("Meow {}", self.name);
    }
}

// using it for polymorphism

fn make_sound(animal: &impl Animal) {
    animal.speak();
}

// same as

fn make_sound_generics<T: Animal>(animal: &T) {
    animal.speak();
}

// trait bounds with different syntaxes

fn print_debug<T: Debug>(obj: &T) {
    println!("{:?}", obj);
}

fn print_display<T: Display + Debug>(obj: &T) {
    println!("{}", obj);
}

fn print_clone<T>(obj: T) -> T where T: Clone + Debug {
    let clone: T = obj.clone();
    println!("{:?}", clone);
    clone
}

fn print_multiple_generics<T: Display, U: Debug + Clone>(obj1: T, obj2: U) {

}

// requiring other traits

trait Printable: Debug {
    fn print(&self) {
        println!("{:?}", self)
    }
}

// associated functions

trait Create {
    fn new() -> Self;
}

struct Player;

impl Create for Player {
    fn new() -> Self {
        Player
    }
}

// associated types (like in the iterator trait)

trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}

struct NumberBox {
    value: i32,
}

impl Container for NumberBox {
    type Item = i32;

    fn get(&self) -> &i32 {
        &self.value
    }
}

// derivable standard lib traits
// debug allows printing with {:?}
// display allows printing with {}
// clone explicit duplication
// copy implicit duplication

// default provides a default value for example for i32 it's 0
#[derive(Default, Debug)]
struct Config {
    retries: i32
}

// PartialEq, Eq to use == and !=
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {
            x, y
        }
    }
}

// PartialOrd, Ord for comparison operators > < >= <= and sorting

struct Distance {
    p1: Point,
    p2: Point
}

// provide a custom implementation

impl Distance {
    fn new(p1: Point, p2: Point) -> Self {
        Distance {
            p1, p2
        }
    }

    fn length_squared(&self) -> i32 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        dx * dx + dy * dy
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.length_squared() == other.length_squared()
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length_squared().cmp(&other.length_squared())
    }
}

// note: this example showcases what we can do but its better to just store the value
// and delegate comparison to a specific field instead of computing every time

// the drop trait (called automatically by rust as well)
struct File;

impl Drop for File {
    fn drop(&mut self) {
        println!("Closing file");
    }
}

// operators are traits like add
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// conversion traits

#[derive(Debug)]
struct MyNumber {
    value: i32
}

impl From<i32> for MyNumber {
    fn from(value: i32) -> Self {
        MyNumber { value }
    }
}

// working with generics and traits bounds in structs at the same time

struct GenericContainer<T> {
    obj: T
}

impl<T: Debug> GenericContainer<T> {
    fn print(&self) {
        println!("{:?}", self.obj)
    }
}

impl<T> Drop for GenericContainer<T> {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

// with where syntax
impl<T> GenericContainer<T>
where T: Display {
    fn print2(&self) {
        println!("{}", self.obj);
    }
}




