use nautile_common::{NautileError, Result};
/// Lifecycle state of a navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationState {
    Idle,
    Started,
    Redirected,
    ResponseStarted,
    Committed,
    Failed,
    Cancelled,
}
/// Request to navigate a frame or tab.
#[derive(Debug, Clone)]
pub struct NavigationRequest {
    pub url: String,
    pub reload_bypass_cache: bool,
}
impl NavigationRequest {
    pub fn new(url: String) -> Self {
        Self {
            url,
            reload_bypass_cache: false,
        }
    }
}
/// Response metadata for a navigation.
#[derive(Debug, Clone)]
pub struct NavigationResponse {
    pub url: String,
    pub mime_type: String,
    pub status: u16,
}
/// Committed document metadata.
#[derive(Debug, Clone)]
pub struct NavigationCommit {
    pub url: String,
    pub title: String,
}
/// Owns session navigation state for a tab/frame.
#[derive(Debug, Clone)]
pub struct NavigationController {
    pub state: NavigationState,
    pub current_url: String,
    pub last_commit: Option<NavigationCommit>,
}
impl Default for NavigationController {
    fn default() -> Self {
        Self {
            state: NavigationState::Idle,
            current_url: "about:blank".into(),
            last_commit: None,
        }
    }
}
impl NavigationController {
    pub fn navigate(&mut self, request: NavigationRequest) -> Result<NavigationCommit> {
        self.state = NavigationState::Started;
        if !(request.url.starts_with("about:")
            || request.url.starts_with("http://")
            || request.url.starts_with("https://")
            || request.url.starts_with("file:"))
        {
            self.state = NavigationState::Failed;
            return Err(NautileError::Unsupported(format!(
                "unsupported URL scheme in {}",
                request.url
            )));
        }
        self.state = NavigationState::ResponseStarted;
        let title = match request.url.as_str() {
            "about:blank" => "Blank",
            "about:version" => "Version",
            "about:crash" => "Crash",
            _ => "Document",
        }
        .to_string();
        let commit = NavigationCommit {
            url: request.url,
            title,
        };
        self.current_url = commit.url.clone();
        self.last_commit = Some(commit.clone());
        self.state = NavigationState::Committed;
        Ok(commit)
    }
}
