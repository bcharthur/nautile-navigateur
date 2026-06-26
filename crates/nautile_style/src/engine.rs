/// CSS style engine.
#[derive(Debug, Default)]
pub struct StyleEngine;
/// Inputs required for style resolution.
#[derive(Debug, Default)]
pub struct StyleContext {
    pub viewport_width: f32,
    pub viewport_height: f32,
}
