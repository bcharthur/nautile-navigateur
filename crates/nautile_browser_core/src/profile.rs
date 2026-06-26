/// User profile state and profile directory selection.
#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            name: "Default".into(),
        }
    }
}
