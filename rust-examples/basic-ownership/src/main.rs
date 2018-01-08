fn print_hello_world() {
    println!("Hello, world!"); // String literal is hard coded into our program, it has no owner.
}

fn print_a_number(number: usize) { // as an argument to our function, number is stored on the stack as a copy.
    println!("Marc is {}, that's old!", number);
} // number falls out of scope and is cleared off the stack along with the rest of this stack frame.

fn print_from_the_heap() {
    let expletive = String::from("shit!"); // "shit!" is allocated on the heap, with `expletive` as its owner
    println!("This is a steaming heap of {}", expletive);
} // expletive is out of scope / dropped, and its spot on the heap made available.

fn main() {
    print_hello_world();
    print_a_number(32);
    print_from_the_heap();
}
