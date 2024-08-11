#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::square(20);
    let rect2 = Rectangle::square(10);
    println!(
        "The area of the rectangle is {} square pixels, and it has a {} width",
        rect1.area(),
        if rect1.width() { "valid" } else { "invalid" }
    );
    println!(
        "rect1 {} hold rect2",
        if rect1.can_hold(&rect2) {
            "can"
        } else {
            "cannot"
        }
    );
    println!(
        "rect2 {} hold rect1",
        if rect2.can_hold(&rect1) {
            "can"
        } else {
            "cannot"
        }
    );

    dbg!(&rect1);
}
