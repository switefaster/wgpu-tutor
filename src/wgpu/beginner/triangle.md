# 七巧板？——图形与三角

我们现在想在平面内确定一个图形，应该怎么做？不妨以三角形为例：

在初中我们学过，确定三边，或者两边一夹角等等就可以确定一个三角形的形状。母庸置疑的是，确定三角形的形状需要至少3个已知量，而为了在平面上确定这个三角形，我们还需要三角形上某一点的坐标以及绕其的旋转角度。坐标需要两个量，所以总共至少需要6个量。换句话说，一个平面内的三角形具有6个自由度，6个线性不相关的量就能准确描述一个三角形。在计算机中，我们会很自然地选择三个顶点的坐标。

同理，四边形具有8个自由度，例如4个顶点的坐标，我们就可以向计算机准确地传达我们需要的形状。

那圆呢？

你或许会说，圆不是只有3个自由度吗？这不简单，原点坐标和半径不就可以了？是，但是为了后续处理的方便，我们需要一个统一的描述方式，比如坐标。那为什么不直接随便选圆上三个点呢（这里出现了6个量，但是这并不影响圆的自由度是3。因为不同的三个点可以确定同一个圆，是对称性使然）？但是这样我们就需要有更多的计算来从这三个点计算出整个圆所占的区域，~~实在是有点憨~~。

最后，我们发现，想要处理起来方便，最好还是横平竖直的东西……

于是，早期计算机图形学家们干脆选择了最简单的方法：都用三角形！那要是边缘看着不够圆滑咋办捏？那就加三角形的量呗……遂，我们就需要用这种类似七巧板的方式，使用微元法的思想，大致拼凑出我们想要的图形。当然，现在已经有各式各样的建模软件会帮我们自动处理这些过程。我们平常说的高模和低模，也是在代指三角形的数量(面数)。

> 当然，三角形并不是我们唯一的选择。图形引擎通常也支持四边形和五边形等，不少游戏也使用混合不同面形状的模型来改善精度。然而不可否认的是，三角形仍然是业界使用最广泛的基本图形。

事不宜迟，我们赶紧试着在窗口里面画一个三角形！

在这之前，我们先引入一个方便我们使用的库`bytemuck`。

```toml
# Cargo.toml

[dependencies]
# ...
bytemuck = "1"
```

这个库将会帮助我们将结构体直接转化为字节slice`&[u8]`，并保证我们的操作是安全的。

我们来定义我们的顶点

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
```

我们定义了一个顶点结构，其中包括顶点的坐标和顶点的颜色。其内容具体如何使用我们将在之后的章节详细讨论。同时注意到我们derive了`bytemuck::Pod`和`bytemuck::Zeroable`两个宏，他们将会允许我们安全地转换我们的结构为`&[u8]`。当然，为此我们还引入了`#[repr(C)]`来保证我们内存的对齐是符合我们预期的。详情请读者自行参见[bytemuck的文档](https://docs.rs/bytemuck)和[Rust Nomicon](https://doc.rust-lang.org/nomicon/)。

接下来，让我们进行一个三角形的编：

```rust
let triangle = [
    Vertex {
        position: [0.0, 0.5],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 1.0, 1.0],
    },
];

let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: None,
    contents: bytemuck::cast_slice(&triangle),
    usage: wgpu::BufferUsages::VERTEX,
});
```

首先我们创建了一个顶点的数组，其次我们将其用`bytemuck`转化为字节后上传到了一个缓冲中。

这里我们首次使用了`Device::create_buffer_init`。事实上，这是一个扩展函数，是不位于WebGPU标准中，WGPU库为了方便使用而添加的一些方法。类似这些的方法被定义在`DeviceExt`这个trait中。

那么，`create_buffer_init`到底为我们干了什么呢？它主要干了这样两件事：

1. 创建一个 __合适__ 大小的缓冲
2. 把我们的数据扔进去

通过翻阅代码（我也鼓励读者如此操作），我们可以发现`create_buffer_init`的所作所为不外乎上面两件事。那么为什么我要强调 __合适__ 大小的缓冲呢？这和Vulkan等WGPU使用的底层API的限定有关。如果我们查看`create_buffer_init`实现中的注释，我们会发现如果要拷贝数据进入缓冲中（换句话说，有`BufferUsages::COPY`），Vulkan要求创建的缓冲的大小必须是`COPY_BUFFER_ALIGNMENT`的倍数。`create_buffer_init`会自动帮我们把缓冲的大小垫（_padding_）到距离我们传入的数据大小大于且最近的满足要求的大小。换句话说，我们通过这个方法创建的缓冲未必就是我们传入的数据的大小！如果读者需要更精确地控制缓冲的大小，我们建议使用`create_buffer`方法并手动上传数据。

接下来，你可能会兴致冲冲地去准备把三角形画出来。可是，WGPU怎么知道怎么理解我们上传的数据呢？如你所见，我们仅仅是上传了很多字节而已。不出意外的话，我们应当得手动描述缓冲的形状（_layout_）才对。

你说得对，接下来我们将开始处理一些棘手的部分，这些部分和后面的章节挂钩。我会尽量通俗地解释这些概念。如果你有什么疑问，读下去，我相信都会得到解答。同时，我也欢迎大家在评论区提问。

首先，我们得创建一个渲染管线（还记得吗，在[之前的章节](../infra/graphics.md)的最后有提到），来告诉WGPU我们将如何渲染这个三角形。

让我们开始吧：

```rust
let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: None,
    bind_group_layouts: &[],
    push_constant_ranges: &[],
});

let shader_module = device.create_shader_module(wgpu::include_wgsl!("triangle/triangle.wgsl"));

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

```

哇塞，真多！而且是不是都是看不懂的新玩意……

对此，我只能说：_你先别急，让我先急_

笔者也是非常的头大的！在这短短（？）的几行代码中，已知与未知交织在一起，令我无从下手。
