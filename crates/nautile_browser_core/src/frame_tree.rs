//! Frame tree for main frame and iframes.
#[derive(Debug, Clone, Default)]
pub struct FrameTree {
    pub frames: Vec<crate::frame::Frame>,
}
