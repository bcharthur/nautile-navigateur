/// Parsed CSS stylesheet.
#[derive(Debug, Clone, Default)]
pub struct Stylesheet {
    pub rules: Vec<crate::CssRule>,
}
