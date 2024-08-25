use outlineprint::OutlinePrint;

fn main() {
    let point = outlineprint::Point { x: 10, y: 8 };
    point.outline_print();
}

pub mod outlineprint {
    use core::fmt;
    use std::fmt::Display;

    pub trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {output} *");
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}
}
/*
 * fn main() {
 *     let beans = Beans {
 *         index: 0,
 *         beans: vec![
 *             Bean {
 *                 smell: String::from("Stinky"),
 *                 color: String::from("Red"),
 *             },
 *             Bean {
 *                 smell: String::from("Very Stinky"),
 *                 color: String::from("Brown"),
 *             },
 *             Bean {
 *                 smell: String::from("Intensly Stinkalicious"),
 *                 color: String::from("Yellow"),
 *             },
 *         ],
 *     };
 *     for bean in beans {
 *         println!("This bean is {} and {}", bean.color, bean.smell)
 *     }
 * }
 *
 * struct Bean {
 *     color: String,
 *     smell: String,
 * }
 * impl Clone for Bean {
 *     fn clone(&self) -> Self {
 *         Bean {
 *             color: self.color.clone(),
 *             smell: self.smell.clone(),
 *         }
 *     }
 * }
 *
 * struct Beans {
 *     beans: Vec<Bean>,
 *     index: usize,
 * }
 *
 * impl Iterator for Beans {
 *     type Item = Bean;
 *
 *     fn next(&mut self) -> Option<Self::Item> {
 *         let current_bean = &self.beans.get(self.index);
 *         let new_bean = current_bean.cloned();
 *         self.index += 1;
 *         new_bean
 *     }
 * }
 */
