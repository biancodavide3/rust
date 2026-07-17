use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub fn main() {
    println!("BEGIN MAP EXAMPLE ------------------------");
    map_example();
    println!("BEGIN SET EXAMPLE ------------------------");
    set_example();
}

fn print_map<K: Debug, V: Debug>(map: &HashMap<K, V>) {
    println!("{:?}", map);
}

fn map_example() {
    let mut ages: HashMap<String, i32> = HashMap::new();
    let alice: String = "Alice".to_string();
    ages.insert(alice.clone(), 20);
    ages.insert("Bob".to_string(), 25);
    print_map(&ages);
    ages.insert(alice.clone(), 21); // adding to the same overrides the value
    // note that we clone because .insert takes ownership of the key
    print_map(&ages);
    // getting a value
    // we can just pass a reference when working with the map (do not clone unnecessarily)
    if let Some(age) = ages.get(&alice) {
        println!("alice age = {}", age);
    }
    // checking that a key exists
    if ages.contains_key(&alice) {
        println!("Found");
    }
    // iterating
    for (k, v) in &ages {   // & not to consume the content of the map
        println!("{} {}", k, v);
    }
    // removing
    let removed: Option<i32> = ages.remove(&alice);
    match removed {
        Some(age) =>  {
            println!("Removed age = {}", age);
        }
        None =>  {
            println!("Key not found");
        }
    }
}

fn print_set<T: Debug>(set: &HashSet<T>) {
    println!("{:?}", set);
}

fn set_example() {
    let mut numbers: HashSet<i32> = HashSet::new();
    numbers.insert(5);
    numbers.insert(3);
    print_set(&numbers);
    let is_inserted: bool = numbers.insert(3); // this value is not added
    print!("the value was inserted = {}", is_inserted);
    // checking if a value exists
    if numbers.contains(&5) {
        println!("value exists");
    }
    // iterating
    for value in &numbers {
        println!("{}", value);
    }
    // removing
    if numbers.remove(&10) {
        println!("some value was removed");
    } else {
        println!("no value was removed");
    }
}