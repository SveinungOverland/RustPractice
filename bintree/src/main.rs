enum NodeComparison {
    MoveLeft,
    MoveRight
}

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {

    fn new(value: i32) -> Node {
        Node{
            value: value,
            left: None,
            right: None,
        }
    }

    fn move_left<'a>(&'a mut self) -> Option<&'a mut Box<Node>> {
        self.left.as_mut()
    }

    fn move_right<'a>(&'a mut self) -> Option<&'a mut Box<Node>> {
        self.right.as_mut()
    }

    fn compare<'a>(&'a mut self, value: i32) -> NodeComparison {
        match self.value {
            it if it > value => NodeComparison::MoveLeft,
            it if it < value => NodeComparison::MoveRight,
            _ => todo!()
        }
    }

    fn insert<'a>(&'a mut self, value: i32){ 
        let start_left_or_right = self.compare(value);
        match start_left_or_right {
            NodeComparison::MoveLeft => if self.left.is_none() { self.left = Some(Box::new(Node::new(value))); return },
            NodeComparison::MoveRight => if self.right.is_none() { self.right = Some(Box::new(Node::new(value))); return }
        }
        let mut temp: &'a mut Box<Node> = match start_left_or_right {
            NodeComparison::MoveLeft => self.left.as_mut().unwrap(),
            NodeComparison::MoveRight => self.right.as_mut().unwrap()
        };
        loop {
            match temp.compare(value) {
                NodeComparison::MoveLeft => {
                    if temp.left.is_none() {
                        temp.left = Some(Box::new(Node::new(value)));
                        return
                    } else {
                        temp = temp.move_left().unwrap()
                    }
                },
                NodeComparison::MoveRight => {
                    if temp.right.is_none() {
                        temp.right = Some(Box::new(Node::new(value)));
                        return
                    } else {
                        temp = temp.move_right().unwrap()
                    }
                }
            }
        }
    }

    fn traverse_down_left_to_end<'a>(&'a mut self) -> &'a mut Box<Node> {
        let mut temp: &'a mut Box<Node> = self.move_left().unwrap();
        while temp.left.is_some() {
            temp = temp.move_left().unwrap();
        }
        temp
    }
}


fn build_node_tree() -> Box<Node> {
    return Box::new(Node{
        value: 10,
        left: Some(Box::new(Node{
            value: 9,
            left: Some(Box::new(Node{
                value: 8,
                left: Some(Box::new(Node{
                    value: 7,
                    left: None,
                    right: None
                })),
                right: None,
            })),
            right: None,
        })),
        right: Some(Box::new(Node{
            value: 20,
            left: Some(Box::new(Node{
                value: 15,
                left: None,
                right: Some(Box::new(Node{
                    value: 17,
                    left: None,
                    right: None,
                }))
            })),
            right: None
        }))
    })
}


fn main() {
    println!("Hello world");
    let mut parent_node = build_node_tree();
    {
        let bottom_left = parent_node.traverse_down_left_to_end();
        println!("Bottom left: {}", bottom_left.value);
        bottom_left.insert(1);
    }
    println!("New bottom left: {}", parent_node.traverse_down_left_to_end().value);
}