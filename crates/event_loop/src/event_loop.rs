use std::collections::HashMap;
use crate::task::{Task, TaskQueue, TaskSource};
use crate::microtask::{Microtask, MicrotaskQueue};

pub struct EventLoop {
    task_queues: HashMap<TaskSource, TaskQueue>,
    microtasks: MicrotaskQueue,
    animation_callbacks: Vec<Box<dyn FnOnce(f64) + Send>>,
    running: bool,
}

impl EventLoop {
    pub fn new() -> Self {
        let mut task_queues = HashMap::new();
        for src in [
            TaskSource::DomManipulation,
            TaskSource::UserInteraction,
            TaskSource::Networking,
            TaskSource::HistoryTraversal,
            TaskSource::Timer,
            TaskSource::Render,
        ] {
            task_queues.insert(src, TaskQueue::default());
        }
        Self { task_queues, microtasks: MicrotaskQueue::default(), animation_callbacks: Vec::new(), running: false }
    }

    pub fn queue_task(&mut self, task: Task) {
        self.task_queues.entry(task.source).or_default().enqueue(task);
    }

    pub fn queue_microtask(&mut self, m: Microtask) {
        self.microtasks.enqueue(m);
    }

    pub fn request_animation_frame(&mut self, cb: impl FnOnce(f64) + Send + 'static) {
        self.animation_callbacks.push(Box::new(cb));
    }

    /// Run one full turn of the event loop.
    pub fn turn(&mut self, timestamp: f64) {
        // 1. Run one task
        let task = self.task_queues.values_mut()
            .find_map(|q| q.dequeue());
        if let Some(task) = task {
            (task.callback)();
        }

        // 2. Drain microtasks
        let microtasks: Vec<_> = self.microtasks.drain_all().collect();
        for m in microtasks {
            (m.callback)();
        }

        // 3. Animation frames
        let cbs: Vec<_> = self.animation_callbacks.drain(..).collect();
        for cb in cbs { cb(timestamp); }
    }
}

impl Default for EventLoop { fn default() -> Self { Self::new() } }
