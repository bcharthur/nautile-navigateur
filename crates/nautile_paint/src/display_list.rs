/// Ordered list of paint commands.
#[derive(Debug, Default, Clone)]
pub struct DisplayList {
    pub items: Vec<crate::DisplayItem>,
}
