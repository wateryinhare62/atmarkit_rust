enum List {
    Node(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: i32) -> List {
        Node(elem, Box::new(self))
    }
}

use List::{Node, Nil};

fn main() {
    let mut list = List::new();
    list = list.prepend(0); 
    list = list.prepend(1); 
    list = list.prepend(2); 

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
