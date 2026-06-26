//! Stable typed identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BrowserId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TabId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FrameId(pub u64);
