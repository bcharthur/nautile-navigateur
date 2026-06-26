/// Root of computed layout boxes.
#[derive(Debug, Default)]
pub struct LayoutTree {
    pub boxes: Vec<crate::LayoutBox>,
}
