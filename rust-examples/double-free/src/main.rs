fn main() {
    let heap_allocation = String::from("Pass me on!");
    let recipient = heap_allocation; // ownership moved from `heap_allocation` to `recipient`

    println!("{}", heap_allocation);
}

// instead of dropping once for each variable, Rust has the concept of moving ownership of heap-allocated data
// no effort is made to run .drop() on `heap_allocation`.
