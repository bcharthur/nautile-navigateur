//! Browser-level state, tabs, frames, profiles, and navigation orchestration.
pub mod bookmarks;
pub mod browser;
pub mod downloads;
pub mod frame;
pub mod frame_tree;
pub mod navigation;
pub mod permissions;
pub mod process_host;
pub mod profile;
pub mod session_history;
pub mod tab;
pub use browser::{Browser, BrowserConfig, BrowserDump};
pub use frame_tree::FrameTree;
pub use navigation::{
    NavigationCommit, NavigationController, NavigationRequest, NavigationResponse, NavigationState,
};
pub use profile::Profile;
pub use tab::Tab;
