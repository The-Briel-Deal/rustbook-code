fn main() {
    for i in 1..5 {
        let i = Some(i);
        match i {
            Some(i) if i % 2 == 0 => println!("{i} is even"),
            Some(i) => println!("{i} is odd"),
            None => (),
        }
    }
}
