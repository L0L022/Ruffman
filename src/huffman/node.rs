//use std::cmp::Ordering;

pub struct Node {
    right: Option<Box<Node>>,
    left: Option<Box<Node>>,
    character: u8,
    frequency: u64,
}

impl Node {
    pub fn new_leaf(character: u8, frequency: u64) -> Node {
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
            character: 0,
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

    pub fn character(&self) -> u8 {
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
/*

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for Node {}
*/
