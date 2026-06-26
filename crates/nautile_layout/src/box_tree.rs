/// Kind of layout box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutBoxKind {
    Block,
    Inline,
    InlineBlock,
    FlexContainer,
    GridContainer,
    Table,
    Replaced,
    Anonymous,
}
/// A box in the layout tree.
#[derive(Debug, Clone)]
pub struct LayoutBox {
    pub kind: LayoutBoxKind,
}
