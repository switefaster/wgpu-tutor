fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let window = winit::window::WindowBuilder::new()
        .with_title("Test Window")
        .build(&event_loop)?;

    event_loop.run(move |event, target| match event {
        winit::event::Event::WindowEvent { window_id, event } if window.id() == window_id => {
            match event {
                winit::event::WindowEvent::CloseRequested => target.exit(),
                _ => (),
            }
        }
        _ => (),
    })?;

    Ok(())
}
