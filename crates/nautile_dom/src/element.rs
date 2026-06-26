/// Public element handle data.
#[derive(Debug, Clone, Default)]
pub struct Element {
    pub id: Option<crate::NodeId>,
}
/// Element node data stored in the DOM arena.
#[derive(Debug, Clone, Default)]
pub struct ElementData {
    pub local_name: String,
    pub attributes: Vec<(String, String)>,
}
