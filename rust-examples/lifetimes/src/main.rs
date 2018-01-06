// This example is taken directly from https://doc.rust-lang.org/stable/book/second-edition/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures

// As the compiler will tell us, this function will need lifetime parameters
// The reason why is because we are taking different slices, from different
// scopes, and we are returning a slice from an undefined scope.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
