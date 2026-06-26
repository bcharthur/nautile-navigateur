use crate::{Task, TaskSource};
/// Minimal Web event loop with task and microtask queues.
#[derive(Debug, Default)]
pub struct WebEventLoop {
    pub tasks: std::collections::VecDeque<Task>,
    pub microtasks: Vec<crate::Microtask>,
    pub rendering_ticks: u64,
}
impl WebEventLoop {
    pub fn enqueue(&mut self, task: Task) {
        self.tasks.push_back(task);
    }
    pub fn input(&mut self, label: impl Into<String>) {
        self.enqueue(Task::new(TaskSource::UserInteraction, label));
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        self.enqueue(Task::new(TaskSource::Rendering, format!("resize {w}x{h}")));
    }
    pub fn render_tick(&mut self) {
        self.rendering_ticks += 1;
        self.enqueue(Task::new(TaskSource::Rendering, "render tick"));
    }
    pub fn close(&mut self) {
        self.enqueue(Task::new(TaskSource::UserInteraction, "close"));
    }
}
