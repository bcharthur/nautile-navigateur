use std::collections::HashMap;
use crate::tab::{Tab, TabId};

pub struct Browser {
    tabs: HashMap<TabId, Tab>,
    next_tab_id: u32,
    active_tab: Option<TabId>,
}

impl Browser {
    pub fn new() -> Self {
        Self { tabs: HashMap::new(), next_tab_id: 1, active_tab: None }
    }

    pub fn create_tab(&mut self) -> TabId {
        let id = TabId::new(self.next_tab_id);
        self.next_tab_id += 1;
        self.tabs.insert(id, Tab::new(id));
        if self.active_tab.is_none() { self.active_tab = Some(id); }
        id
    }

    pub fn close_tab(&mut self, id: TabId) {
        self.tabs.remove(&id);
        if self.active_tab == Some(id) {
            self.active_tab = self.tabs.keys().next().copied();
        }
    }

    pub fn navigate(&mut self, id: TabId, url: impl Into<String>) {
        if let Some(tab) = self.tabs.get_mut(&id) {
            let url = url.into();
            tab.url = url.clone();
            tab.loading = true;
            tab.nav_state = crate::navigation::NavigationState::Started { url };
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_tab.and_then(|id| self.tabs.get(&id))
    }

    pub fn tabs(&self) -> impl Iterator<Item = &Tab> {
        self.tabs.values()
    }
}

impl Default for Browser { fn default() -> Self { Self::new() } }
