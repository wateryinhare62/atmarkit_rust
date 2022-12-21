enum List {
    Node(i32, Box<List>),
    Nil,
}

use List::{Node, Nil};

fn main() {
    let list = Node(0, 
        Box::new(Node(1, 
            Box::new(Node(2, 
                Box::new(Nil))))));
    let mut l = &list;
    loop {
        match &*l {
            Nil => {
                println!("Nil");
                break;
            },
            Node(value, next) => {
                println!("{}", value);
                l = &next;
            },
        }
    }
}