fn make_cool(lame: &mut String) {
    lame.push_str(", yo");
}

fn main() {
    let mut devops = String::from("dev"); // adding `mut` means we make a variable mutable
    devops.push_str("ops");

    make_cool(&mut devops); // we can add `mut` to references to make them mutable as well

    println!("{}", devops);
}
