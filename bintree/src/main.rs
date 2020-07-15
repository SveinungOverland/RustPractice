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

    fn has_left(&self) -> bool {
        self.left.is_some()
    }

    fn has_right(&self) -> bool {
        self.right.is_some()
    }

    fn move_left<'a>(&'a mut self) -> Option<&'a mut Box<Node>> {
        self.left.as_mut()
    }

    fn move_right<'a>(&'a mut self) -> Option<&'a mut Box<Node>> {
        self.right.as_mut()
    }

    fn set_left(&mut self, node: Node) {
        self.left = Some(Box::new(node))
    }

    fn set_right(&mut self, node: Node) {
        self.right = Some(Box::new(node))
    }

    fn compare<'a>(&'a mut self, value: i32) -> NodeComparison {
        match self.value {
            it if it > value => NodeComparison::MoveLeft,
            it if it < value => NodeComparison::MoveRight,
            _ => todo!()
        }
    }

    

    fn _direct_insert_or_child<'a>(&'a mut self, value: i32) -> Option<&'a mut Box<Node>> {
        let comparison = self.compare(value);
        {
            let has_child: bool = match comparison { NodeComparison::MoveLeft => self.has_left(), NodeComparison::MoveRight => self.has_right() };
            if has_child {
                return match comparison { NodeComparison::MoveLeft => self.move_left(), NodeComparison::MoveRight => self.move_right() }
            };
        }
        let new_node = Node::new(value);
        match comparison {
            NodeComparison::MoveLeft => self.set_left(new_node),
            NodeComparison::MoveRight => self.set_right(new_node)
        }
        None
    }

    fn insert<'a>(&'a mut self, value: i32) {
        let mut temp: &'a mut Box<Node> = match self._direct_insert_or_child(value) {
            Some(child) => child,
            None => return
        };
        loop { 
            temp = match temp._direct_insert_or_child(value) {
                Some(child) => child,
                None => return
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