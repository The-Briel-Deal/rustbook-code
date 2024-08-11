fn main() {
    println!("Hello, world!");
    let x = another_function(10);
    println!("Return is {x}");
}

fn another_function(x: i32) -> i32 {
    let y = {
        let x = 3;
        x + 2
    };
    return x + y;
}
