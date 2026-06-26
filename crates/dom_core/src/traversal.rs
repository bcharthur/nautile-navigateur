use crate::arena::{DomArena, NodeId};
use crate::node::Node;

pub struct PreOrderIter<'a> {
    arena: &'a DomArena,
    stack: Vec<NodeId>,
}

impl<'a> PreOrderIter<'a> {
    pub fn new(arena: &'a DomArena, root: NodeId) -> Self {
        Self { arena, stack: vec![root] }
    }
}

impl<'a> Iterator for PreOrderIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        let id = self.stack.pop()?;
        let node = self.arena.get(id);
        let mut child = node.last_child;
        while let Some(c) = child {
            self.stack.push(c);
            child = self.arena.get(c).previous_sibling;
        }
        Some(node)
    }
}
