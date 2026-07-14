use std::fmt::Debug;

fn main() {
    println!("BEGIN array_example ------------------------");
    array_example();
    println!("BEGIN vec_example --------------------------");
    vec_example();
    println!("BEGIN slice_example -------------------------");
    slice_example();
}

// using a slice
fn print<T: Debug>(vec: &[T]) {
    println!("{:?}", vec)
}

// it is a view on existing data useful to pass arrays or vectors to functions
fn slice_example() {
    // try using it in place first
    let mut example: Vec<i32> = vec![1, 2, 3, 4];
    let mut_slice: &mut [i32] = &mut example[1..3]; // 1 included up to 3 excluded,\
    // remove 3 and leave it at 1.. for everything to the end or similarly ..3
    for x in mut_slice {
        *x *= 10;
    }
    let immut_slice: &[i32] = &example[..3];
    println!("Before it was copied");
    print(immut_slice);
    let mut container: Vec<i32> = Vec::new();
    for x in immut_slice {
        container.push(*x); // doesn't move the value it copies
    }
    println!("After it was copied");
    print(immut_slice);
    print(&example);
    // now with another function
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numbers2: [i32; 5] = [0, 2, 4, 6, 8];
    println!("Before");
    print(&numbers);
    slice_example_continue(&mut numbers, &numbers2);
    println!("After");
    print(&numbers);
    print(&numbers2);
}

fn slice_example_continue(mutable: &mut [i32], immutable: &[i32]) {
    for x in mutable {
        *x *= 10;
    }
    for x in immutable {
        println!(" in function {}", *x);
    }
}

// fixed size
fn array_example() {
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50];
    print(&arr);
    for x in &mut arr {
        *x += 1;
    }
    for x in &arr {
        println!("{}", x); // display trait to print an i32
    }
}

// dynamic with len and capacity
fn vec_example() {
    // let mut numbers: Vec<i32> = Vec::new();
    let mut numbers: Vec<i32> = vec![1,2,3,4];
    numbers.push(5);
    println!("{:?}", numbers); // debug trait to print a Vec
    // getting a value
    let first: i32 = numbers[0];
    // let x: i32 = numbers[100]; panic
    match numbers.get(100) {
        Some(value) => println!("{}", value),
        None => println!("Not Found")
    };
    // iterating
    // immutable
    for x in &numbers {
        println!("{}", x);
    }
    // mutable
    for x in &mut numbers {
        *x += 1;
    }
    // do not do this as numbers will be moved in the loop and not be accessible later
    /*
    for x in numbers {
        println!("{}", x)
    }
     */
    // as its like doing this
    // let mut iterator = numbers.into_iter();
    // while let Some(x) = iterator.next() {
    //     println!("{}", x);
    // }
    println!("{:?}", numbers);

    numbers.remove(2);
    println!("{:?}", numbers);
    println!("{}", numbers.len());
    let mut numbers2: Vec<i32> = vec![7, 8];
    numbers.append(&mut numbers2); // MOVES the contents of the second array into the first
    println!("{:?}", numbers);
    println!("{:?}", numbers2);
    // or you can use extend that takes any iterable
    numbers.extend(vec![9, 10]);
    println!("{:?}", numbers);
    println!("{}", numbers.capacity()); // differs from length because it is what rust has allocated
    numbers.remove(numbers.len()-1);
    println!("{:?}", numbers);
    println!("{}", numbers.capacity());
    // iterating it again
    let immutable_iterable: &Vec<i32> = &numbers; // we can store the & in a variable
    for num in immutable_iterable {
        println!("{}", num);
    }
    let mutable_iterable: &mut Vec<i32> = &mut numbers;
    for num in mutable_iterable {
        // playing with the pointers a bit
        let old = *num;
        if old != 5 {
            *num += 1;
        }
        println!("old: {}, new: {}", old, *num);
    }
}