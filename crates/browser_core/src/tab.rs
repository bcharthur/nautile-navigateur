use crate::navigation::NavigationState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TabId(u32);

impl TabId {
    pub fn new(v: u32) -> Self { Self(v) }
    pub fn get(self) -> u32 { self.0 }
}

#[derive(Debug)]
pub struct Tab {
    pub id: TabId,
    pub title: String,
    pub url: String,
    pub favicon: Option<Vec<u8>>,
    pub loading: bool,
    pub nav_state: NavigationState,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

impl Tab {
    pub fn new(id: TabId) -> Self {
        Self {
            id,
            title: "Nouvel onglet".into(),
            url: "about:newtab".into(),
            favicon: None,
            loading: false,
            nav_state: NavigationState::Idle,
            can_go_back: false,
            can_go_forward: false,
        }
    }
}
