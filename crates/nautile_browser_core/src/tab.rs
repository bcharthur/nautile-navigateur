use crate::{FrameTree, NavigationController};
use nautile_common::ids::TabId;
/// Browser tab containing a frame tree and navigation controller.
#[derive(Debug, Clone)]
pub struct Tab {
    pub id: TabId,
    pub current_url: String,
    pub frame_tree: FrameTree,
    pub navigation: NavigationController,
}
impl Tab {
    pub fn new(id: TabId) -> Self {
        Self {
            id,
            current_url: "about:blank".into(),
            frame_tree: FrameTree::default(),
            navigation: NavigationController::default(),
        }
    }
}
