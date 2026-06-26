use crate::arena::NodeId;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub last_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
    pub previous_sibling: Option<NodeId>,
    pub data: NodeData,
}

#[derive(Debug, Clone)]
pub enum NodeData {
    Document(DocumentData),
    Element(ElementData),
    Text(TextData),
    Comment(String),
    DocumentType(DocumentTypeData),
    DocumentFragment,
}

#[derive(Debug, Clone, Default)]
pub struct DocumentData {
    pub url: Option<String>,
    pub charset: String,
    pub mode: DocumentMode,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum DocumentMode {
    #[default]
    NoQuirks,
    Quirks,
    LimitedQuirks,
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub namespace: Namespace,
    pub local_name: String,
    pub attributes: Vec<Attribute>,
    pub is_self_closing: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Namespace {
    Html,
    Svg,
    MathMl,
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub namespace: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TextData {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct DocumentTypeData {
    pub name: String,
    pub public_id: String,
    pub system_id: String,
}
