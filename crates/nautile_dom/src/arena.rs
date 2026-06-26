use crate::{Node, NodeData, NodeId};
/// Arena storing DOM nodes with stable integer ids.
#[derive(Debug, Default, Clone)]
pub struct DomArena {
    nodes: Vec<Node>,
}
impl DomArena {
    pub fn create(&mut self, data: NodeData) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(Node {
            id,
            parent: None,
            children: vec![],
            data,
        });
        id
    }
    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id.0 as usize)
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
