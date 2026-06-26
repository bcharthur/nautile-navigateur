use crate::{Task, TaskSource};

/// Accumulated scroll position in CSS pixels for the active browsing context.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ScrollState {
    pub x: f32,
    pub y: f32,
}

/// Minimal Web event loop with task and microtask queues.
#[derive(Debug, Default)]
pub struct WebEventLoop {
    pub tasks: std::collections::VecDeque<Task>,
    pub microtasks: Vec<crate::Microtask>,
    pub rendering_ticks: u64,
    pub scroll: ScrollState,
}
impl WebEventLoop {
    pub fn enqueue(&mut self, task: Task) {
        self.tasks.push_back(task);
    }
    pub fn input(&mut self, label: impl Into<String>) {
        self.enqueue(Task::new(TaskSource::UserInteraction, label));
    }
    pub fn scroll_by(&mut self, delta_x: f32, delta_y: f32) {
        self.scroll.x = (self.scroll.x + delta_x).max(0.0);
        self.scroll.y = (self.scroll.y + delta_y).max(0.0);
        self.enqueue(Task::new(
            TaskSource::UserInteraction,
            format!("scroll dx={delta_x:.2} dy={delta_y:.2}"),
        ));
        self.enqueue(Task::new(TaskSource::Rendering, "scroll invalidation"));
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

#[cfg(test)]
mod tests {
    use super::WebEventLoop;

    #[test]
    fn scroll_updates_state_and_schedules_rendering() {
        let mut event_loop = WebEventLoop::default();
        event_loop.scroll_by(0.0, 120.0);
        assert_eq!(event_loop.scroll.y, 120.0);
        assert_eq!(event_loop.tasks.len(), 2);
    }
}
