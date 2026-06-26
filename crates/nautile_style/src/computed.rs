/// Computed style values for one element.
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    pub display: String,
}
/// DOM node with computed style.
#[derive(Debug, Clone, Default)]
pub struct StyledNode {
    pub node_id: u32,
    pub style: ComputedStyle,
}
