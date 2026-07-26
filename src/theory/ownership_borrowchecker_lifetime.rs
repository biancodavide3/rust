// Ownership is the central idea in Rust
// Almost everything else like borrowing, lifetimes, String vs &str, Vec, Box, Rc, Arc,
// and even the borrow checker exists because of ownership.

pub fn main() {
    // every value can only have 1 owner
    {
        let s = String::from("hello");
    }
    // the value is dropped automatically (i.e. the drop trait is called)

    // values stored directly in the stack are small
    let x = 5;
    let y = x;
    // the value of x is also copied into y
    // this works because i32 implements the Copy trait
    // these types include i32, u32, bool, char, f64, usize
    println!("{x}"); // x is still accessible

    // for more complex trait we have a move of the value
    let s = String::from("hello");
    let mut s2 = s;
    // now the value of s is moved to s2 and ownership is transferred and s is now useless

    // in this case if you want two copies you can clone
    let s3 = s2.clone();
    // now both s2 and s3 are holding the same value but are different variables
    // this is because String implements the Clone trait
    // every copy type also implements clone
    // so every copy type is also a clone trait (e.g. i32)
    // but not all clone traits are copy types (e.g. string)

    // when you are passing values to functions it's exactly like variable assignment
    // as well as in loops, structs or anything really
    // so it all depends on the type you are passing
    let s4 = s2.clone();
    print(s4);
    // s4 is not accessible anymore here ... as it's moved into the function

    // most functions just need to borrow the value they generally do not need to take ownership

    // when you are borrowing a value with a & (immutable) or a &mut (mutable) borrow
    // the borrow checker enforces either that you can exclusively have either:
    // any number of immutable references
    // 1 and only 1 mutable reference

    // ok
    let r1 = &s3;
    let r2 = &s3;
    let r3 = &s3;

    // ok
    let w1 = &mut s2;

    // a lifetime is the period during which a REFERENCE is valid
    // the key idea is that a reference can never live longer than the owner of the value it points to
    // the compiler already tracks lifetimes with scopes but we can specify them explicitly
    // you can specify them explicitly with letters such as 'a 'b kind of like generics
}

// -----------------------------------------
// LIFETIME FUNCTIONS

// compiler cannot know from signature alone which one we could choose to return to we
// say that the returned reference has the fame lifetime as both parameters
// (1 parameter would be trivial)
fn choose<'a>(
    x: &'a String,
    y: &'a String,
) -> &'a String {
    x
}

// if we were to use multiple lifetime each parameter has its own lifetime and
// they are completely independent, we are not enforcing anything
fn example<'a, 'b>(
    first: &'a String,
    second: &'b String,
) {
    println!("{first}");
    println!("{second}");
}

// can also be used in structs
struct Book<'a> {
    title: &'a str,
}

// but most of the time the compiler just inserts it automatically

// the syntax can be confusing if you are also using generics
struct Holder<'a, T> {
    value: &'a T,
}

// 'static is a special lifetime

// -----------------------------------------
// OWNERSHIP FUNCTIONS

fn print(s: String) -> () {
    println!("{}", s);
}

// this one doesn't compile because the value of s is dropped at the end of the function
// but are still trying to return a reference to it so rust doesn't compile

/*
fn dangling_reference() -> &String {
    let s = String::from("hello");
    &s
}
*/