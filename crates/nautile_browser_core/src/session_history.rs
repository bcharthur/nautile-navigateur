//! Session history support.
#[derive(Debug, Clone, Default)]
pub struct SessionHistory {
    pub entries: Vec<String>,
}
