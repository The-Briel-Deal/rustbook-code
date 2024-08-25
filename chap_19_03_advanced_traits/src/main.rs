fn main() {
    let beans = Beans {
        index: 0,
        beans: vec![
            Bean {
                smell: String::from("Stinky"),
                color: String::from("Red"),
            },
            Bean {
                smell: String::from("Very Stinky"),
                color: String::from("Brown"),
            },
            Bean {
                smell: String::from("Intensly Stinkalicious"),
                color: String::from("Yellow"),
            },
        ],
    };
    for bean in beans {
        println!("This bean is {} and {}", bean.color, bean.smell)
    }
}

struct Bean {
    color: String,
    smell: String,
}
impl Clone for Bean {
    fn clone(&self) -> Self {
        Bean {
            color: self.color.clone(),
            smell: self.smell.clone(),
        }
    }
}

struct Beans {
    beans: Vec<Bean>,
    index: usize,
}

impl Iterator for Beans {
    type Item = Bean;

    fn next(&mut self) -> Option<Self::Item> {
        let current_bean = &self.beans.get(self.index);
        let new_bean = current_bean.cloned();
        self.index += 1;
        new_bean
    }
}
