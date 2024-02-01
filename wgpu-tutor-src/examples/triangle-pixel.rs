use pollster::FutureExt;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let window = winit::window::WindowBuilder::new()
        .with_title("Test Window")
        .build(&event_loop)?;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = instance.create_surface(&window)?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .block_on()
        .unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,                           // 如果你给他起个名字，调试的时候可能比较有用
            required_features: adapter.features(), // 根据需要的特性自行调整
            required_limits: adapter.limits(),     // 根据需要的限定自行调整
        },
        None,
    ))
    .unwrap();

    let capabilities = surface.get_capabilities(&adapter);

    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: capabilities.formats[0],
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &surface_config);

    // 在屏幕左下角，底边、高都是100px的三角形
    let triangle = [
        Vertex {
            position: [50.0, 100.0],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.0, 0.0],
            color: [1.0, 1.0, 1.0],
        },
        Vertex {
            position: [100.0, 0.0],
            color: [1.0, 1.0, 1.0],
        },
    ];

    let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&triangle),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let window_dimension = window.inner_size();

    let pixel_matrix = cgmath::Matrix4::new(
        2.0 as f32 / window_dimension.width as f32,
        0.,
        0.,
        0.,
        0.,
        2.0 / window_dimension.height as f32,
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        -1.,
        -1.,
        0.,
        1.,
    );

    let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&<cgmath::Matrix4<f32> as Into<[[f32; 4]; 4]>>::into(
            pixel_matrix,
        )),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let matrix_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let matrix_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &matrix_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: projection_buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&matrix_bind_group_layout],
        push_constant_ranges: &[],
    });

    let shader_module =
        device.create_shader_module(wgpu::include_wgsl!("triangle/triangle-pixel.wgsl"));

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vs_main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as _,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![
                    0 => Float32x2,
                    1 => Float32x3
                ],
            }],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });

    event_loop.run(|event, target| {
        match event {
            winit::event::Event::WindowEvent { event, window_id } if window.id() == window_id => {
                match event {
                    winit::event::WindowEvent::Resized(new_size) => {
                        if new_size.width > 0 && new_size.height > 0 {
                            surface_config.width = new_size.width;
                            surface_config.height = new_size.height;
                            surface.configure(&device, &surface_config);

                            let pixel_matrix = cgmath::Matrix4::new(
                                2.0 as f32 / new_size.width as f32,
                                0.,
                                0.,
                                0.,
                                0.,
                                2.0 / new_size.height as f32,
                                0.,
                                0.,
                                0.,
                                0.,
                                1.,
                                0.,
                                -1.,
                                -1.,
                                0.,
                                1.,
                            );
                            queue.write_buffer(
                                &projection_buffer,
                                0,
                                bytemuck::cast_slice(&<cgmath::Matrix4<f32> as Into<
                                    [[f32; 4]; 4],
                                >>::into(
                                    pixel_matrix
                                )),
                            );
                        }
                    }
                    winit::event::WindowEvent::CloseRequested => target.exit(),
                    winit::event::WindowEvent::RedrawRequested => {
                        let output = surface.get_current_texture().unwrap();
                        let view = output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder = device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                        {
                            // 注意这个
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                            render_pass.set_pipeline(&pipeline);
                            render_pass.set_vertex_buffer(0, vertices_buffer.slice(..));
                            render_pass.set_bind_group(0, &matrix_bind_group, &[]);
                            render_pass.draw(0..3, 0..1);
                        }
                        queue.submit(std::iter::once(encoder.finish()));
                        output.present();
                    }
                    _ => (),
                }
            }
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        }
    })?;

    Ok(())
}
