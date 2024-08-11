enum Animals {
    Dog(String),
    Cat(String),
}
fn main() {
    let doggy = Animals::Dog(String::from("Naru"));
    let kitty = Animals::Cat(String::from("Butter"));
    greet(doggy);
    greet(kitty);

    println!("Hello, world!");
}

fn greet(animal: Animals) {
    match animal {
        Animals::Cat(str) => {
            println!("I'm a cat named {str}!")
        }
        Animals::Dog(str) => {
            println!("I'm a dog named {str}!")
        }
    }
}
