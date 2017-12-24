fn main() {
    let true_story =  vec!["Devops is great"];
    println!("Some char in true_story: {}", true_story[5]); // tries to access an area of memory that is out of bounds.
} // this example does not compile, Rust has built in guards against this type of attempt.
