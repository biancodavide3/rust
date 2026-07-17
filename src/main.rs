mod theory;

// examples of use at different level (can only traverse down in the hierarchy)
use std::sync::Arc;
use std::sync::mpsc::channel;

// if you need to go up (for example you are in a submodule and you need code in another submodule)
// you can specify the absolute path with use crate::

// using our lib
use hello_rust;

// modules can be declared also inside the same file
pub mod hello {
    pub fn greet() {
        println!("hello");
    }
}


fn main() {
    // println!("{}", hello_rust::square(3));
    // vec_arr_slice::main();
    // hello::greet()
}

