fn dangle<'a>() -> &'a str {
    let world = String::from(", world!");
    return &world
}

// fn dangle<'a>() -> &'static str {
//     return ", world!"
// } // toggle to demo static lifetimes.

fn main() {
    println!("Hello{}", dangle());
}
