use crate::{cli::Cli, window::GpuWindow};
use nautile_browser_core::{Browser, BrowserConfig};
use nautile_event_loop::WebEventLoop;
use std::error::Error;
use winit::{
    event::{ElementState, Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};
/// Runs the desktop shell.
pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Nautile Navigateur")
        .build(&event_loop)?;
    let mut browser = Browser::new(BrowserConfig::default());
    let tab = browser.create_tab();
    browser.navigate_tab(tab, cli.url)?;
    let mut web_loop = WebEventLoop::default();
    let mut gpu = GpuWindow::new(&window)?;
    event_loop.run(|event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    web_loop.close();
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    web_loop.resize(size.width, size.height);
                    gpu.resize(size);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        web_loop.input("keyboard");
                        if matches!(event.logical_key, Key::Named(NamedKey::Escape)) {
                            elwt.exit();
                        }
                    }
                }
                WindowEvent::CursorMoved { .. } => web_loop.input("mouse move"),
                WindowEvent::MouseInput { .. } => web_loop.input("mouse button"),
                WindowEvent::MouseWheel { delta, .. } => {
                    let label = match delta {
                        MouseScrollDelta::LineDelta(_, y) => format!("scroll lines {y}"),
                        MouseScrollDelta::PixelDelta(p) => format!("scroll pixels {}", p.y),
                    };
                    web_loop.input(label);
                }
                WindowEvent::RedrawRequested => {
                    web_loop.render_tick();
                    match gpu.render() {
                        Ok(()) => {}
                        Err(wgpu::SurfaceError::Lost) => gpu.resize(window.inner_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(_) => {}
                    }
                }
                _ => {}
            },
            Event::AboutToWait => window.request_redraw(),
            _ => {}
        }
    })?;
    Ok(())
}
