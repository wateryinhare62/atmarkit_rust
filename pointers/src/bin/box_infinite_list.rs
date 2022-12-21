enum List {
    Node(i32, List),
    Nil,
}

use List::{Node, Nil};

fn main() {
    let list = Node(0, 
        Node(1, 
            Node(2, 
                Nil)));
}