# 一切的起点——初始化

本小节，我们将正式开始初始化WGPU，并且用其给我们的窗口染上颜色。

## 准备工作

当然，在开始之前我们需要添加一些依赖！

```toml
# Cargo.toml

[package]
# ...
resolver = "2" #!IMPORTANT 这对 wgpu >= 0.10 是必要的
# UPDATE: 从rust edition 2021开始 resolver = 2 是缺省的

[dependencies]
winit = "0.29"
wgpu = "0.19"
pollster = "0.3"
```

可以看到，除了`winit`外，我们迎来了两位新朋友。第一位当然是我们的主角`wgpu`，而出于对 **WebGPU** 规范的遵从，`wgpu`包含了少量异步(`async`)函数。当然我们希望我们能将这些异步函数像同步函数一样处理(当然这会不可避免地造成阻塞，有异步需求的同学请自行摸索)，因而我们需要有一个`Executor`实现来阻塞地运行某个`Future`，出于方便，我们选择了`pollster`，其提供了一个简单的`pollster::block_on`函数。

准备完毕，我们可以开始了。

## 正式开始

看过上一节以及上上节的读者应该已经知道，WGPU的一切应当从`wgpu::Instance`开始。WGPU的代码都相当直接，所以我们先直接看代码吧。

```rust,no_run
use pollster::FutureExt; // 有了这个我们就可以对任意Future使用block_on()了

fn main() {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let window = winit::window::WindowBuilder::new()
        .with_title("Test Window")
        .build(&event_loop)?;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        flags: wgpu::InstanceFlags::default(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    // ...
}
```

`wgpu::InstanceDescriptor`描述了我们需要创建的实例的基本信息。`wgpu::Backends`是一个`BitSet`，每个不同的位表示了尝试使用这个后端(_1_)与否(_0_)。当然，如果你全选了WGPU通常会根据你的系统自动帮你挑一个，有需要的就自己指定吧。剩下的字段请读者自行翻看文档，我们不是很关心，所以都使用默认值即可。接下来，我们有了实例，该获取`Surface`了

```rust,no_run
// ...
let surface = instance.create_surface(&window).unwrap();
let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    })
    .block_on()
    .unwrap();
```

我们接着获取适配器，`request_adapter`这个函数会让`Instance`帮你挑一个满足你要求的适配器。好像参数也没啥好解释的……`wgpu::PowerPreference`会决定WGPU倾向于选择独显还是集显，看你咯。

当然，如果你想自己枚举适配器，也是可以的。方法大致如下：

```rust,no_run
let adapter = instance
    .enumerate_adapters(wgpu::Backends::all())
    .filter(|adapter| {
        // 筛选你需要的适配器
    })
    .first()
    .unwrap()

```

好，底层干部基本就位了，接下来我们就要请出我们的一线工人`Queue`和`Device`。

```rust,no_run
let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,                  // 如果你给他起个名字，调试的时候可能比较有用
            required_features: adapter.features(), // 根据需要的特性自行调整
            required_limits: adapter.limits(),     // 根据需要的限定自行调整
        },
        None,
    )
    .block_on()
    .unwrap();
```

简单、直白、明了。当然，我们理所当然地注意到了`Features`和`Limits`。`Features`是显卡设备需要支持的功能，例如深度裁剪等等。而`Limits`是一些喜闻乐见的限制，比如允许创建的材质的数量之类。当WGPU无法创建满足条件的设备时，会果断丢出一个`Error`。当然，我们这里方便起见直接`unwrap()`了=_=

当然，如果你没在这里声明你需要用到的功能而在后面的程序中使用到了，则WGPU会在执行此功能时panic。这样一定程度上避免了WGPU在不同的设备上有不同的行为。

万事俱备！……吗？我们好像还没告诉WGPU咱们的帧缓冲得多大啊……

```rust,no_run
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
```

我们先获取了我们的`Surface`的`SurfaceCapabilities`，其中包含了我们的设备和平面支持的像素格式、呈现模式和alpha值模式。
我们将`Surface`的帧缓冲配置为我们窗口的大小，并告诉他我们的帧缓冲可以用来当`RENDER_ATTACHMENT`，人话说就是可以当渲染目标的东西。然后给他挑了个他能用的像素格式。查询 `PresentMode` 的文档你会发现有多种模式，详情请参照文档。其中几种是 **垂直同步** 的，也就是说当窗口需要被显示时程序会等到该帧被完全显示，通常这取决于显示屏的刷新率，这会减少画面割裂的产生。而最后一种则是立即显示，这种情况下最能反应当前设备下能达到的最优帧率。虽然不一定好就是了。`view_formats`则规定了我们创建帧缓冲视图时可以使用哪些格式。而`desired_maximum_frame_latency`则会决定交换链将提前渲染多少帧的内容，我们这里设为`2`，那么交换链维护的样子就是我们上章所示了。是的，视图的格式可以和缓冲本身不同。但是我们通常只会用到格式相同的情况，而这种情况永远都是被支持的，所以我们留空就行了。

这下真万事俱备了，但是我们还需要对我们的循环做一点小调整。

```rust,no_run
event_loop.run(|event, target| {
    target.set_control_flow(ControlFlow::Wait);

    match event {
        winit::event::Event::WindowEvent { event, window_id } if window.id() == window_id => {
            match event {
                winit::event::WindowEvent::Resized(new_size) => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface_config.width = new_size.width;
                        surface_config.height = new_size.height;
                        surface.configure(&device, &surface_config);
                        }
                }
                winit::event::WindowEvent::CloseRequested => target.exit(),
                winit::event::WindowEvent::RedrawRequested => {
                    // 在这渲染
                }
                _ => (),
            }
        }
        winit::event::Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    }
}).unwrap();
```

上面的代码，说人话，就是在窗口大小变化的时候重新配置一下咱们的`Surface`。熟悉了渲染流程的读者可能已经猜到，如果不这么做，很有可能导致巨大的窗口上只寥寥显示了几个巨大的像素的惨剧……

## 万事俱备，终于……

开始我们的渲染吧~

渲染一个窗口需要几步？<mask>好几步</mask>

1. 获取要渲染对象的视图
2. 发送渲染命令
3. 把渲染命令丢件队列里面

这就是我们的代码将要做的。

```rust,no_run
let output = surface.get_current_texture().unwrap();
let view = output
    .texture
    .create_view(&wgpu::TextureViewDescriptor::default());
let mut encoder = device
    .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
{
                            // 注意这个 '{'
    let _render_pass =
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
    }
queue.submit(std::iter::once(encoder.finish()));
output.present();
```

也挺直白的，不是么？

我们来看看我们到底做了啥。

首先，我们向`Surface`请求了交换链中的下一个帧缓冲，然后创建了它的视图。接下来。我们创建了一个命令编码器，来创建一个命令缓冲。然后，我们开始了一个渲染阶段，并且告诉WGPU我们需要渲染到哪些`RENDER_ATTACHMENT`上面，顺便告诉WGPU怎么清除这些渲染对象，包括在读取的时候清除并设置默认值，然后对其的操作要写入到缓冲中。`resolve_target` 是MSAA可能会使用到的技术，我们在此不做赘述。`depth_stencil_attachment` 将在后文提及并使用。

然后，我们暂时不用进行别的渲染操作，所以我们让编码器创建了个命令缓冲，然后把它丢进了队列。

> 为了强调`Queue::submit`接受的是迭代器，我使用了`std::iter::once`，然而我们实际上可以直接使用`Some(encoder.finish())`，因为`Option<T>`实现了`IntoIterator<Item=T>`

**SO EZ!**

另外值得我们注意的是，为了内存安全需要，当然也是因为其内部操作的必要，`RenderPass` 内部保留着一个 `&mut CommandEncoder`，换而言之，我们的编码器的数据是流入`RenderPass`中的，因此我们需要稍微控制其生命周期，以防止Rust编译器对你狂暴鸿儒大量错误。这也是为什么我们打了一组看似多余的`{}`。

不出意外的话，我们的程序现在可以顺利运行，并且你会看到一片绿到发光的屏幕<mask>没有暗讽各位读者的意思，大概</mask>。于是，我们正式迈出了使用WGPU的第一步。不过请注意，我们激动人心的旅程才刚刚开始！
