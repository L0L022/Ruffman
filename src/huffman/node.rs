struct Node {
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
    character: char,
    frequency: u64,
}

impl Node {
    pub fn new_leaf(character: char, frequency: u64) -> Node {
        Node {
            right: None,
            left: None,
            character: character,
            frequency: frequency
        }
    }

    pub fn new_branch(right: Node, left: Node) -> Node {
        Node {
            right: Some(Box::new(right)),
            left: Some(Box::new(left)),
            character: '\0',
            frequency: right.frequency() + left.frequency()
        }
    }

    pub fn character(&self) -> char {
        self.character
    }

    pub fn frequency(&self) -> u64 {
        self.frequency
    }

    pub fn is_leaf(&self) -> bool {
        self.right == None && self.left == None
    }

    pub fn is_branch(&self) -> bool {
        self.right != None || self.left != None
    }
}
