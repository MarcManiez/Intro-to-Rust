fn dangle<'a>() -> &'a str {
    let world = String::from(", world!");
    return &world
}

fn main() {
    println!("Hello{}", dangle());
}
