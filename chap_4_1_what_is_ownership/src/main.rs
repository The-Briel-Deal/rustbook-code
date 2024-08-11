fn main() {
    let mut s = String::from("hello");
    let mrs = &mut s;
    append_world(mrs);
    let mrs2 = &s;
    println!("{mrs2}");
}

fn append_world(s: &mut String) {
    s.push_str(", world!");
}
