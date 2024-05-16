use std::sync::Arc;

use pollster::FutureExt;
use winit::application::ApplicationHandler;

struct Application<'a> {
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[derive(Default)]
struct State<'a> {
    app: Option<Application<'a>>,
}

impl<'a> ApplicationHandler for State<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(winit::window::Window::default_attributes().with_title("窗口标题"))
                .unwrap(),
        );

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,                           // 如果你给他起个名字，调试的时候可能比较有用
                    required_features: adapter.features(), // 根据需要的特性自行调整
                    required_limits: adapter.limits(),     // 根据需要的限定自行调整
                },
                None,
            )
            .block_on()
            .unwrap();

        let capabilities = surface.get_capabilities(&adapter);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);
        self.app = Some(Application {
            window,
            surface,
            surface_config,
            device,
            queue,
        })
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(Application {
            window,
            surface,
            surface_config,
            device,
            queue,
        }) = &mut self.app
        {
            if window.id() != window_id {
                return;
            }
            match event {
                winit::event::WindowEvent::CloseRequested => event_loop.exit(),
                winit::event::WindowEvent::Resized(new_size) => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface_config.width = new_size.width;
                        surface_config.height = new_size.height;
                        surface.configure(&device, &surface_config);
                    }
                }
                winit::event::WindowEvent::RedrawRequested => {
                    let output = surface.get_current_texture().unwrap();
                    let view = output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                    {
                        // 注意这个 '{'
                        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                    }
                    queue.submit(std::iter::once(encoder.finish()));
                    output.present();
                    window.request_redraw();
                }
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
