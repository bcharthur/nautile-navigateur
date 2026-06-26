//! Browser-oriented event loop queues.
pub mod animation_frame;
pub mod event_loop;
pub mod microtask;
pub mod render_step;
pub mod task;
pub mod task_queue;
pub mod timer;
pub use event_loop::WebEventLoop;
pub use microtask::Microtask;
pub use task::{Task, TaskSource};
