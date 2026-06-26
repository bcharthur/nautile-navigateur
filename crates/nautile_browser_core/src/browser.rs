use crate::{NavigationRequest, Profile, Tab};
use nautile_common::{ids::TabId, version, Result};
/// Configuration used to create a browser instance.
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    pub product_name: String,
    pub version: String,
    pub headless: bool,
}
impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            product_name: version::PRODUCT_NAME.into(),
            version: version::NAUTILE_VERSION.into(),
            headless: false,
        }
    }
}
/// Top-level browser object for single-process V0 and future browser process host.
#[derive(Debug)]
pub struct Browser {
    pub config: BrowserConfig,
    pub profile: Profile,
    pub tabs: Vec<Tab>,
    next_tab_id: u64,
}
/// Textual dump of observable browser state.
#[derive(Debug, Clone)]
pub struct BrowserDump {
    pub product: String,
    pub tabs: Vec<String>,
}
impl Browser {
    pub fn new(config: BrowserConfig) -> Self {
        Self {
            config,
            profile: Profile::default(),
            tabs: Vec::new(),
            next_tab_id: 1,
        }
    }
    pub fn create_tab(&mut self) -> TabId {
        let id = TabId(self.next_tab_id);
        self.next_tab_id += 1;
        self.tabs.push(Tab::new(id));
        id
    }
    pub fn navigate_tab(&mut self, id: TabId, url: impl Into<String>) -> Result<()> {
        let tab = self.tabs.iter_mut().find(|t| t.id == id).ok_or_else(|| {
            nautile_common::NautileError::InvalidInput(format!("unknown tab {}", id.0))
        })?;
        tab.navigation
            .navigate(NavigationRequest::new(url.into()))?;
        tab.current_url = tab.navigation.current_url.clone();
        Ok(())
    }
    pub fn dump_state(&self) -> BrowserDump {
        BrowserDump {
            product: format!("{} {}", self.config.product_name, self.config.version),
            tabs: self
                .tabs
                .iter()
                .map(|t| {
                    format!(
                        "tab={} url={} state={:?}",
                        t.id.0, t.current_url, t.navigation.state
                    )
                })
                .collect(),
        }
    }
}
