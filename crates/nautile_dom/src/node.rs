/// Stable DOM node identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);
/// DOM node stored in the arena.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub data: NodeData,
}
/// Concrete data carried by a DOM node.
#[derive(Debug, Clone)]
pub enum NodeData {
    Document,
    Element(crate::ElementData),
    Text(String),
    Comment(String),
    DocumentFragment,
}
