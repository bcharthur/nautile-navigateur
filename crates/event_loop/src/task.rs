use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskSource {
    DomManipulation,
    UserInteraction,
    Networking,
    HistoryTraversal,
    Timer,
    Render,
    Microtask,
}

pub struct Task {
    pub source: TaskSource,
    pub callback: Box<dyn FnOnce() + Send>,
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task").field("source", &self.source).finish()
    }
}

impl Task {
    pub fn new(source: TaskSource, callback: impl FnOnce() + Send + 'static) -> Self {
        Self { source, callback: Box::new(callback) }
    }
}

#[derive(Default)]
pub struct TaskQueue {
    inner: VecDeque<Task>,
}

impl TaskQueue {
    pub fn enqueue(&mut self, task: Task) { self.inner.push_back(task); }
    pub fn dequeue(&mut self) -> Option<Task> { self.inner.pop_front() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
}
