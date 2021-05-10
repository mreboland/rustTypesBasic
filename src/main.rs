fn main() {
    println!("Hello, world!");
}

// Rust is a statically typed language. Without running the program, the compiler checks that every possible path of execution will use values only in ways consistent with their types.
// Compared to a dynamically typed language like JavaScript or Python, Rust requires more planning from you up front: you must spell out the types of functions’ parameters and return values, members of struct types, and a few other constructs.

// Rust will infer most types for you. For example, if we were to spell out every type in a function:
fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}
// The above is cluttered and repetitive. Looking at the function's return type, it's obvious that v must be a Vec<i16>, a vector of 16-bit signed integers. Nothing else would work. As such, we can simplify it down as Rust will infer the i16 in the rest of the program:

fn build_vector_inferred() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

