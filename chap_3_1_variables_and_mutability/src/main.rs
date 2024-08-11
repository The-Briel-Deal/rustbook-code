const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    let x = 5;
    println!("The value of x is {x}");
    {
        let x = 6;
        println!("The value of x is {x}");
    }
    println!("The value of x is {x}");
    println!("The hours in seconds is {THREE_HOURS_IN_SECONDS}");
}
