/// Source category for a web task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskSource {
    UserInteraction,
    Networking,
    DomManipulation,
    HistoryTraversal,
    Timer,
    Rendering,
}
/// A queued browser task.
#[derive(Debug, Clone)]
pub struct Task {
    pub source: TaskSource,
    pub label: String,
}
impl Task {
    pub fn new(source: TaskSource, label: impl Into<String>) -> Self {
        Self {
            source,
            label: label.into(),
        }
    }
}
