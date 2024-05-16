use std::sync::Arc;

use winit::application::ApplicationHandler;

struct Application {
    window: Arc<winit::window::Window>,
}

#[derive(Default)]
struct State {
    app: Option<Application>,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app = Some(Application {
            window: Arc::new(
                event_loop
                    .create_window(
                        winit::window::Window::default_attributes().with_title("窗口标题"),
                    )
                    .unwrap(),
            ),
        })
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(Application { window }) = &mut self.app {
            if window.id() != window_id {
                return;
            }
            match event {
                winit::event::WindowEvent::CloseRequested => event_loop.exit(),
                _ => (),
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut state = State::default();

    event_loop.run_app(&mut state)?;

    Ok(())
}
