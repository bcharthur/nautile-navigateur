//! DOM arena and node model.
pub mod arena;
pub mod attr;
pub mod comment;
pub mod document;
pub mod element;
pub mod mutation;
pub mod node;
pub mod query;
pub mod serialization;
pub mod text;
pub mod traversal;
pub mod tree;
pub use arena::DomArena;
pub use document::Document;
pub use element::{Element, ElementData};
pub use node::{Node, NodeData, NodeId};
pub use text::Text;
