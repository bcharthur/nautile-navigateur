use crate::arena::{DomArena, NodeId};
use crate::node::{DocumentData, NodeData};

pub struct Document {
    pub arena: DomArena,
    pub root: NodeId,
    pub head: Option<NodeId>,
    pub body: Option<NodeId>,
}

impl Document {
    pub fn new() -> Self {
        let mut arena = DomArena::new();
        let root = arena.allocate(NodeData::Document(DocumentData {
            url: None,
            charset: "UTF-8".into(),
            mode: Default::default(),
        }));
        Self { arena, root, head: None, body: None }
    }
}

impl Default for Document {
    fn default() -> Self { Self::new() }
}
