// lib.rs has no main
// a project (hello-rust) can have main, lib and other modules (vec_arr_slice)
// main and its modules compose a binary crate, while lib a library crate
// main will depend on any library crate it uses that is compiled on its own (so other binary crates can use it)
// but a lib can also be tested without a main (exam structure)
// "use" is a shortcut not to have to write things as std::sync::Arc::new(...) but just new(...) and it works if the
// module you specify is exposing what you are trying to use (pub)
// "mod" tells our main.rs that the module vec_arr_slice exists and, therefore it can call its functions
// you have to put "pub" before members declaration to make them visible one level beyond its current privacy boundary
// for example in std::sync::Arc
// std is a crate in the standard library with a module called sync that contains the Arc struct
// but there can be more nesting
// such as std::sync::mpsc::channel() where we have to also specify pub mod mpsc in sync to be able to use its members like channel()

// private constant
const PI: f64 = 3.1415;

// public functions
pub fn square(x: i32) -> i32 {
    x * x
}

pub fn circumference(r: f64) -> f64 {
    2.0 * r * PI
}

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(3), 9);
        assert_eq!(square(-2), 4);
    }
}