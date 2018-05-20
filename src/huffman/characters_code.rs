use std::collections::BTreeMap;
use huffman::tree::Tree;

pub type CharactersCode = BTreeMap<u8, Vec<bool>>;

pub fn new_characters_code(tree: &Tree) -> CharactersCode {
    let mut cc = CharactersCode::new();
    let mut nodes = vec![(tree.root(), Vec::new())];
    
    while !nodes.is_empty() {
        let mut new_nodes = Vec::new();
        
        for (node, code) in nodes {
            if node.is_leaf() {
                cc.insert(node.character(), code);
            } else if node.is_branch() {
                new_nodes.push((node.left().unwrap(), code.clone()));
                new_nodes.last_mut().unwrap().1.push(false);
                new_nodes.push((node.right().unwrap(), code));
                new_nodes.last_mut().unwrap().1.push(true);
            }
        }
        
        nodes = new_nodes;
    }
    
    cc
}
