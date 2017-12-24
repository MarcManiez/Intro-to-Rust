fn main() {
    let hello_world = String::from("Hello, world!"); // hello_world is allocated on the heap. malloc() equivalent

    println!("{}", hello_world);

} // hello_world is out of scope and automatically deallocated as a result. No need to run free()
