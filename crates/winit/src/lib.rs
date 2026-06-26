pub mod dpi {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PhysicalSize<T> {
        pub width: T,
        pub height: T,
    }
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PhysicalPosition<T> {
        pub x: T,
        pub y: T,
    }
}
pub mod keyboard {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum NamedKey {
        Escape,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Key {
        Named(NamedKey),
        Character(String),
    }
}
pub mod event_loop {
    use super::event::Event;
    pub struct EventLoop;
    pub struct EventLoopWindowTarget {
        exit: std::cell::Cell<bool>,
    }
    #[derive(Debug, Clone, Copy)]
    pub enum ControlFlow {
        Poll,
        Wait,
    }
    impl EventLoopWindowTarget {
        pub fn set_control_flow(&self, _: ControlFlow) {}
        pub fn exit(&self) {
            self.exit.set(true)
        }
    }
    impl EventLoop {
        pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self)
        }
        pub fn run<F>(self, mut f: F) -> Result<(), Box<dyn std::error::Error>>
        where
            F: FnMut(Event, &EventLoopWindowTarget),
        {
            let t = EventLoopWindowTarget {
                exit: std::cell::Cell::new(false),
            };
            f(Event::AboutToWait, &t);
            Ok(())
        }
    }
}
pub mod event {
    use super::{dpi::PhysicalPosition, keyboard::Key};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ElementState {
        Pressed,
        Released,
    }
    #[derive(Debug, Clone)]
    pub struct KeyEvent {
        pub state: ElementState,
        pub logical_key: Key,
    }
    #[derive(Debug, Clone, Copy)]
    pub enum MouseScrollDelta {
        LineDelta(f32, f32),
        PixelDelta(PhysicalPosition<f64>),
    }
    #[derive(Debug, Clone)]
    pub enum WindowEvent {
        CloseRequested,
        Resized(super::dpi::PhysicalSize<u32>),
        KeyboardInput { event: KeyEvent },
        CursorMoved {},
        MouseInput {},
        MouseWheel { delta: MouseScrollDelta },
        RedrawRequested,
    }
    #[derive(Debug, Clone)]
    pub enum Event {
        WindowEvent { event: WindowEvent, window_id: u64 },
        AboutToWait,
    }
}
pub mod window {
    use super::{dpi::PhysicalSize, event_loop::EventLoop};
    #[derive(Debug)]
    pub struct Window {
        id: u64,
        size: PhysicalSize<u32>,
    }
    impl Window {
        pub fn id(&self) -> u64 {
            self.id
        }
        pub fn inner_size(&self) -> PhysicalSize<u32> {
            self.size
        }
        pub fn request_redraw(&self) {}
    }
    pub struct WindowBuilder {
        title: String,
    }
    impl WindowBuilder {
        pub fn new() -> Self {
            Self {
                title: String::new(),
            }
        }
        pub fn with_title(mut self, t: &str) -> Self {
            self.title = t.into();
            self
        }
        pub fn build(self, _: &EventLoop) -> Result<Window, Box<dyn std::error::Error>> {
            Ok(Window {
                id: 1,
                size: PhysicalSize {
                    width: 1280,
                    height: 720,
                },
            })
        }
    }
}
