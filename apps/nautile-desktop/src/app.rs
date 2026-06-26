use crate::{cli::Cli, window::GpuWindow};
use nautile_browser_core::{Browser, BrowserConfig};
use nautile_common::version;
use nautile_event_loop::WebEventLoop;
use std::error::Error;
use winit::{
    event::{ElementState, Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

const LINE_SCROLL_PX: f32 = 40.0;

/// Runs the desktop shell.
pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    if cli.print_version {
        println!("{}", Cli::version_text());
        return Ok(());
    }
    let event_loop = EventLoop::new()?;
    let title = version::browser_version_string();
    let window = WindowBuilder::new().with_title(&title).build(&event_loop)?;
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
                    let (dx, dy) = scroll_delta_to_css_px(delta);
                    web_loop.scroll_by(dx, dy);
                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    web_loop.render_tick();
                    match gpu.render(web_loop.scroll.y) {
                        Ok(()) => {}
                        Err(wgpu::SurfaceError::Lost) => gpu.resize(window.inner_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(_) => {}
                    }
                }
                WindowEvent::Other => {}
            },
            Event::AboutToWait => window.request_redraw(),
            _ => {}
        }
    })?;
    Ok(())
}

fn scroll_delta_to_css_px(delta: MouseScrollDelta) -> (f32, f32) {
    match delta {
        MouseScrollDelta::LineDelta(x, y) => (-x * LINE_SCROLL_PX, -y * LINE_SCROLL_PX),
        MouseScrollDelta::PixelDelta(p) => (-p.x as f32, -p.y as f32),
    }
}
