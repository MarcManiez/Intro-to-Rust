fn dangle<'a>() -> &'a str { // Brief note on lifetimes: used to express the age of values relatively to one another
    let world = String::from(", world!");
    return &world // attempts to return a pointer to the string declared above
} // let's see what the compiler has to say about this...

// fn dangle<'a>() -> &'static str {
//     return ", world!"
// } // toggle to demo static lifetimes.

fn main() {
    println!("Hello{}", dangle());
}
