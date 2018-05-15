pub struct Node {
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

    pub fn new_branch(left: Node, right: Node) -> Node {
        let freq = right.frequency + left.frequency;
        Node {
            right: Some(Box::new(right)),
            left: Some(Box::new(left)),
            character: '\0',
            frequency: freq
        }
    }
    
    pub fn right(&self) -> Option<&Node> {
        match &self.right {
            Some(v) => Some(&v),
            None => None
        }
    }
    
    pub fn left(&self) -> Option<&Node> {
        match &self.left {
            Some(v) => Some(&v),
            None => None
        }
    }

    pub fn character(&self) -> char {
        self.character
    }

    pub fn frequency(&self) -> u64 {
        self.frequency
    }
    
    pub fn is_leaf(&self) -> bool {
        self.right.is_none() && self.left.is_none()
    }

    pub fn is_branch(&self) -> bool {
        self.right.is_some() || self.left.is_some()
    }
}
