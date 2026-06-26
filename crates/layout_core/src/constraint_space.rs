use crate::geometry::Length;

/// Contraintes données à un fragment pour son layout.
#[derive(Debug, Clone, Copy)]
pub struct ConstraintSpace {
    pub available_inline: AvailableSize,
    pub available_block: AvailableSize,
    pub writing_mode: WritingMode,
}

#[derive(Debug, Clone, Copy)]
pub enum AvailableSize {
    Definite(Length),
    Indefinite,
    MinContent,
    MaxContent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WritingMode { HorizontalTb, VerticalRl, VerticalLr }

impl ConstraintSpace {
    pub fn fixed(inline: f32, block: f32) -> Self {
        Self {
            available_inline: AvailableSize::Definite(inline),
            available_block: AvailableSize::Definite(block),
            writing_mode: WritingMode::HorizontalTb,
        }
    }
}
