//! Common error types.
#[derive(Debug, Clone)]
pub enum NautileError {
    Unsupported(String),
    InvalidInput(String),
    Subsystem(String),
}
impl std::fmt::Display for NautileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(s) => write!(f, "unsupported operation: {s}"),
            Self::InvalidInput(s) => write!(f, "invalid input: {s}"),
            Self::Subsystem(s) => write!(f, "subsystem error: {s}"),
        }
    }
}
impl std::error::Error for NautileError {}
pub type Result<T> = std::result::Result<T, NautileError>;
