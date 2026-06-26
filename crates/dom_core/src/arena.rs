use crate::node::{Node, NodeData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u32);

impl NodeId {
    pub fn as_usize(self) -> usize { self.0 as usize }
}

#[derive(Debug, Default)]
pub struct DomArena {
    nodes: Vec<Node>,
}

impl DomArena {
    pub fn new() -> Self { Self::default() }

    pub fn allocate(&mut self, data: NodeData) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(Node {
            id,
            parent: None,
            first_child: None,
            last_child: None,
            next_sibling: None,
            previous_sibling: None,
            data,
        });
        id
    }

    pub fn get(&self, id: NodeId) -> &Node {
        &self.nodes[id.as_usize()]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.as_usize()]
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        {
            
            let prev_last = self.get(parent).last_child;

            if let Some(prev) = prev_last {
                self.get_mut(prev).next_sibling = Some(child);
                self.get_mut(child).previous_sibling = Some(prev);
            } else {
                self.get_mut(parent).first_child = Some(child);
            }
            self.get_mut(parent).last_child = Some(child);
        }
        self.get_mut(child).parent = Some(parent);
    }

    pub fn children(&self, id: NodeId) -> ChildIter<'_> {
        ChildIter { arena: self, next: self.get(id).first_child }
    }

    pub fn len(&self) -> usize { self.nodes.len() }
}

pub struct ChildIter<'a> {
    arena: &'a DomArena,
    next: Option<NodeId>,
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        let id = self.next?;
        let node = self.arena.get(id);
        self.next = node.next_sibling;
        Some(node)
    }
}
