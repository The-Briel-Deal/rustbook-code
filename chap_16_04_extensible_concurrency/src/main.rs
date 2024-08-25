pub mod animals {
    pub struct Dog {
        pub paw_color: String,
        pub girthyness: u8,
    }
}
use animals::Dog;
fn main() {
    let dog = Dog {
        paw_color: "brown_town".to_string(),
        girthyness: 10,
    };
    let _girth = dog.girthyness;
    println!("Hello, world!");
}
