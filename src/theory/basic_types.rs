pub fn main() {
    println!("BEGIN STRING EXAMPLE ----------------------");
    string_example();
    println!("BEGIN NUMERIC TYPES EXAMPLE ---------------");
    numeric_types_example();
    println!("BEGIN BOOL TUPLES EXAMPLE -----------------");
    bool_tuples_example();
}

const PI: f64 = 3.1415;
static COUNTER: i32 = 0;

fn string_example() {
    // String owns memory, can grow and be modified kind of like Vec<T>
    let mut s: String = String::from("hello");
    s.push_str(" world");
    s.push('!');
    println!("{}", s);
    // &str is a string slice and cannot be modified as such
    let s2: &str = "hello";
    // converting &str to String
    let s3 = s2.to_string();
    // converting String to &str
    let s4: &str = &s2;
    // ownership s3's value is now moved in s5
    let s5 = s3;
    // you can clone to have two independent strings
    let s6 = s5.clone();
    // you can slice different portion of the string
    let slice: &str = &s6[2..4];
    println!("{}", slice);
    // common api
    println!("{}", slice.starts_with("l"));
    println!("{}", slice.len() == 2);
    println!("{}", slice.contains("world"));
    // functions usually take &str as parameter for the same reason as &[]
}

fn numeric_types_example() {
    // signed integers: i8, i16, i32, i64, i128, isize
    let num: i32 = -728;
    // unsigned integers: u8, u16, u32, u64, u128, usize
    let index: usize = 12;
    // floating point: f32, f64
    let pi: f64 = 3.1415;
    // conversion
    let x: i32 = 5;
    let y: f64 = 2.0;
    let z: f64 = x as f64 + y;
    println!("{}", z);
}

fn bool_tuples_example() -> () {
    let is_tasty: bool = true;
    let name: String = "Apple".to_string();
    let count: i8 = 12;
    let fruit = (is_tasty, name.clone(), count);
    println!("{}", is_tasty);
    println!("{}", count);
    println!("{}", name);   // notice that we clone and the other values are copied instead otherwise the string would be moved
    println!("{:?}", fruit);
    // destructuring
    let (v1, v2, v3) = fruit;
    println!("{} {} {}", v1 == is_tasty, v2 == name, v3 == count);
    // special () type which means nothing
    // this function returns it for example
    // const exist globally immutable and have a type
    // static also have fixed memory
}