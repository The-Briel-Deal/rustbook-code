enum Node {
    Root {
        left_child: Box<Node>,
        right_child: Box<Node>,
        index: u8,
    },
    Split {
        left_child: Box<Node>,
        right_child: Box<Node>,
        index: u8,
    },
    Leaf {
        index: u8,
    },
}
fn main() {
    let root = Node::Root {
        left_child: Box::new(Node::Split {
            left_child: Box::new(Node::Leaf { index: 3 }),
            right_child: Box::new(Node::Leaf { index: 4 }),
            index: 1,
        }),
        right_child: Box::new(Node::Split {
            left_child: Box::new(Node::Split {
                index: 5,
                left_child: Box::new(Node::Leaf { index: 7 }),
                right_child: Box::new(Node::Leaf { index: 8 }),
            }),
            right_child: Box::new(Node::Leaf { index: 6 }),
            index: 2,
        }),
        index: 0,
    };
    let root = Box::new(root);
    let mut vec = vec![&root];

    let mut string = String::from("");
    while vec.len() != 0 {
        let mut level_size = vec.len();
        while level_size != 0 {
            level_size -= 1;
            let node = vec.pop();
            let node = node.expect("We already checked if the length was 0 in the loop");
            let node = node.as_ref();
            match node {
                Node::Root {
                    left_child,
                    right_child,
                    index,
                } => {
                    string.push_str(&format!("R{index} "));
                    vec.insert(0, left_child);
                    vec.insert(0, right_child);
                }
                Node::Split {
                    left_child,
                    right_child,
                    index,
                } => {
                    string.push_str(&format!("S{index} "));
                    vec.insert(0, left_child);
                    vec.insert(0, right_child);
                }
                Node::Leaf { index } => string.push_str(&format!("L{index} ")),
            }
        }
        string.push_str("\n");
    }
    println!("{string}");
}
