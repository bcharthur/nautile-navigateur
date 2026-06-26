use std::collections::VecDeque;

pub struct Microtask {
    pub callback: Box<dyn FnOnce() + Send>,
}

impl Microtask {
    pub fn new(callback: impl FnOnce() + Send + 'static) -> Self {
        Self { callback: Box::new(callback) }
    }
}

#[derive(Default)]
pub struct MicrotaskQueue {
    inner: VecDeque<Microtask>,
}

impl MicrotaskQueue {
    pub fn enqueue(&mut self, task: Microtask) { self.inner.push_back(task); }
    pub fn drain_all(&mut self) -> impl Iterator<Item = Microtask> + '_ {
        self.inner.drain(..)
    }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
}
