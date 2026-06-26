/// Arbres layout — à construire à partir du style tree.
pub struct LayoutBoxId(pub u32);

pub enum LayoutBoxKind {
    Block,
    Inline,
    InlineBlock,
    FlexContainer,
    FlexItem,
    GridContainer,
    GridItem,
    TableWrapper,
    Anonymous,
    Replaced,
}

pub struct LayoutBox {
    pub id: LayoutBoxId,
    pub node_id: Option<u32>,
    pub kind: LayoutBoxKind,
    pub children: Vec<LayoutBoxId>,
}
