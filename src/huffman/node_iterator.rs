use huffman::node::Node;

pub struct NodeIterator<'a> {
    node: &'a Node
}

impl<'a> NodeIterator<'a> {
    pub fn new(node: &Node) -> NodeIterator {
        NodeIterator {
            node: node
        }
    }
    
    pub fn next(&mut self, direction: bool) -> Option<&Node> {
        let next = match direction {
            true => self.node.right(),
            false => self.node.left(),
        };
        
        match next {
            Some(v) => { self.node = v; Some(v) },
            None => None,
        }
    }
}
