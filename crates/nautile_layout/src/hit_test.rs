/// Result of layout/compositor hit testing.
#[derive(Debug, Clone, Default)]
pub struct HitTestResult {
    pub node: Option<u32>,
}
