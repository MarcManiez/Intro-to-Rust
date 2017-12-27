fn make_cool(lame: &mut String) {
    lame.push_str(", yo");
}

fn main() {
    let mut devops = String::from("dev");
    devops.push_str("ops");

    make_cool(&mut devops);

    println!("{}", devops);
}
