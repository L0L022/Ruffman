use huffman::node::Node;
use huffman::characters_frequency::CharactersFrequency;

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new(cf: &CharactersFrequency) -> Tree {
        // panic if cf is empty
        
        let mut nodes = Vec::with_capacity(cf.len());
        
        for (&character, &frequency) in cf.iter() {
            nodes.push(Node::new_leaf(character, frequency));
        }
        
        while nodes.len() > 1 {
            nodes.sort_by(|a, b| a.frequency().cmp(&b.frequency()));
            let first = nodes.remove(0);
            let second = nodes.remove(0);
            nodes.push(Node::new_branch(first, second));
        }
        
        Tree {
            root: nodes.remove(0)
        }
    }
    
    pub fn root(&self) -> &Node {
        &self.root
    }
}
