/// CSS rule with selectors and declarations.
#[derive(Debug, Clone, Default)]
pub struct CssRule {
    pub selectors: Vec<crate::Selector>,
    pub declarations: Vec<Declaration>,
}
/// CSS property declaration.
#[derive(Debug, Clone, Default)]
pub struct Declaration {
    pub name: String,
    pub value: crate::CssValue,
    pub important: bool,
}
