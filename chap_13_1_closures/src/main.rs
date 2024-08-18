#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 7, height: 12 },
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
    ];

    let mut call_count = 0;

    list.sort_by_key(|r| {
        call_count += 1;
        r.width
    });
    println!("{list:#?} closure was called {call_count} times");
}
