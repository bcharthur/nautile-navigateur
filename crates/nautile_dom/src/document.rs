/// DOM document root.
#[derive(Debug, Clone)]
pub struct Document {
    pub arena: crate::DomArena,
    pub root: crate::NodeId,
}
impl Default for Document {
    fn default() -> Self {
        let mut arena = crate::DomArena::default();
        let root = arena.create(crate::NodeData::Document);
        Self { arena, root }
    }
}
