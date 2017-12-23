fn main() {
    let mut the_room = String::from("Oh hi Mark!");
    the_room.push_str(" Bye Denny.");
    let reference_to_the_room = &the_room; // comment this line and the code will work
    let mutable_reference_to_the_room = &mut the_room;
    mutable_reference_to_the_room.push_str(" You're tearing me apart, Lisa!");
    println!("{}", mutable_reference_to_the_room);
}

// Rust won't compile this code to avoid data races
// Data races occur when two or more pointers, one of which can mutate the data, try to read a location in memory