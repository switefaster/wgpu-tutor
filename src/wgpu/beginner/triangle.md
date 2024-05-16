# 七巧板？——图形与三角

## 理论

我们现在想在平面内确定一个图形，应该怎么做？不妨以三角形为例：

在初中我们学过，确定三边，或者两边一夹角等等就可以确定一个三角形的形状。母庸置疑的是，确定三角形的形状需要至少3个已知量，而为了在平面上确定这个三角形，我们还需要三角形上某一点的坐标以及绕其的旋转角度。坐标需要两个量，所以总共至少需要6个量。换句话说，一个平面内的三角形具有6个自由度，6个线性不相关的量就能准确描述一个三角形。在计算机中，我们会很自然地选择三个顶点的坐标。

同理，四边形具有8个自由度，例如4个顶点的坐标，我们就可以向计算机准确地传达我们需要的形状。

那圆呢？

你或许会说，圆不是只有3个自由度吗？这不简单，原点坐标和半径不就可以了？是，但是为了后续处理的方便，我们需要一个统一的描述方式，比如坐标。那为什么不直接随便选圆上三个点呢（这里出现了6个量，但是这并不影响圆的自由度是3。因为不同的三个点可以确定同一个圆，是对称性使然）？但是这样我们就需要有更多的计算来从这三个点计算出整个圆所占的区域，~~实在是有点憨~~。

最后，我们发现，想要处理起来方便，最好还是横平竖直的东西……

于是，早期计算机图形学家们干脆选择了最简单的方法：都用三角形！那要是边缘看着不够圆滑咋办捏？那就加三角形的量呗……遂，我们就需要用这种类似七巧板的方式，使用微元法的思想，大致拼凑出我们想要的图形。当然，现在已经有各式各样的建模软件会帮我们自动处理这些过程。我们平常说的高模和低模，也是在代指三角形的数量(面数)。

> 当然，三角形并不是我们唯一的选择。图形引擎通常也支持四边形和五边形等，不少游戏也使用混合不同面形状的模型来改善精度。然而不可否认的是，三角形仍然是业界使用最广泛的基本图形。

## 上手操作！

事不宜迟，我们赶紧试着在窗口里面画一个三角形！

在这之前，我们先引入一个方便我们使用的库`bytemuck`。

```toml
# Cargo.toml

[dependencies]
# ...
bytemuck = "1"
```

这个库将会帮助我们将结构体直接转化为字节数组切片`&[u8]`，并保证我们的操作是安全的。

我们来定义我们的顶点

```rust,no_run
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
```

我们定义了一个顶点结构，其中包括顶点的坐标和顶点的颜色。其内容具体如何使用我们将在之后的章节详细讨论。同时注意到我们derive了`bytemuck::Pod`和`bytemuck::Zeroable`两个宏，他们将会允许我们安全地转换我们的结构为`&[u8]`。当然，为此我们还引入了`#[repr(C)]`来保证我们内存的对齐是符合我们预期的。详情请读者自行参见[bytemuck的文档](https://docs.rs/bytemuck)和[Rust Nomicon](https://doc.rust-lang.org/nomicon/)。

接下来，让我们进行一个三角形的编：

```rust,no_run
// resumed

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

> 有些读者可能会好奇为何我们为三角形的顶点坐标选择了这几个数值。粗略地说，在WGPU中，我们会把一个$\left[-1,1\right]\times\left[-1,1\right]$中的点线性映射到窗口的像素中。具体而言，$\left(0,0\right)$会被平移到窗口中央，而$\hat{x}$和$\hat{y}$分别对应屏幕右方向和上方向。在渲染过程中涉及的各个坐标系将会在后面的章节详细讲述。

这里我们首次使用了`Device::create_buffer_init`。事实上，这是一个扩展函数，是不位于WebGPU标准中，WGPU库为了方便使用而添加的一些方法。类似这些的方法被定义在`DeviceExt`这个trait中。

那么，`create_buffer_init`到底为我们干了什么呢？它主要干了这样两件事：

1. 创建一个 **合适** 大小的缓冲
2. 把我们的数据扔进去

通过翻阅代码（我也鼓励读者如此操作），我们可以发现`create_buffer_init`的所作所为不外乎上面两件事。那么为什么我要强调 **合适** 大小的缓冲呢？这和Vulkan等WGPU使用的底层API的限定有关。如果我们查看`create_buffer_init`实现中的注释，我们会发现如果要拷贝数据进入缓冲中（换句话说，有`BufferUsages::COPY`），Vulkan要求创建的缓冲的大小必须是`COPY_BUFFER_ALIGNMENT`的倍数。`create_buffer_init`会自动帮我们把缓冲的大小垫（_padding_）到距离我们传入的数据大小大于且最近的满足要求的大小。换句话说，我们通过这个方法创建的缓冲未必就是我们传入的数据的大小！如果读者需要更精确地控制缓冲的大小，我们建议使用`create_buffer`方法并手动上传数据。

接下来，你可能会兴致冲冲地去准备把三角形画出来。可是，WGPU怎么知道怎么理解我们上传的数据呢？如你所见，我们仅仅是上传了很多字节而已。不出意外的话，我们应当得手动描述缓冲的形状（_layout_）才对。

你说得对，接下来我们将开始处理一些棘手的部分，这些部分和后面的章节挂钩。我会尽量通俗地解释这些概念。如果你有什么疑问，读下去，我相信都会得到解答。同时，我也欢迎大家在评论区提问。

首先，我们得创建一个渲染管线（还记得吗，在[之前的章节](../infra/graphics.md)的最后有提到），来告诉WGPU我们将如何渲染这个三角形。

让我们开始吧：

```rust,no_run
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

这段代码涉及了众多之后才能理解的概念，我会尽量使用通俗的语言描述。

首先我们需要为管线创建一个布局（_layout_），这个布局描述了我们之后将会向管线的着色器内传入什么样的额外数据。这里额外数据指任何顶点数据以外的用户指定的数据。由于我们现在并不需要向着色器内传入额外的数据，我们先全部留空。

接下来，我们将我们的着色器文件载入到GPU中。在[之前的章节](../infra/graphics.md)最后对渲染管线的描述中我们提到过，着色器是一种告诉GPU如何完成传入的顶点数据的变化和如何给像素着色等任务的语言。在接下来的几个章节中我们将会更详细地了解着色器。创建好着色器以后我们会得到着色器模块（_ShaderModule_），相当于对GPU内着色器资源的引用。在示例创建着色器的过程中，我们使用了`wgpu::include_wgsl!`这个宏。这个宏会在 **编译期** 将你的着色器载入到程序中，并用其创建一个`wgpu::ShaderModuleDescriptor`。如果你翻阅了[ShaderModuleDescriptor的文档](https://docs.rs/wgpu/latest/wgpu/struct.ShaderModuleDescriptor.html)，会发现其中的`wgpu::ShaderSource`支持相当数量的着色器语言。归功于[naga项目](https://github.com/gfx-rs/naga)，不同的着色器语言最终都会被转译成WGPU可以理解的格式。在接下来的教程中，我们只会使用`wgsl`。

接下来我们就在正式创建渲染管线了。因为用途平凡或内容不会涉及，我们将不会在此深入解释下面几个字段的意义：

- label
- layout
- multisample
- multiview

让我们把注意力放到剩下的几个字段上：

_vertex_ 会用来描述该渲染管线对顶点数据的处理，也就是[渲染流程概要](../infra/graphics.md#渲染流程概要)里前两个节点之间的部分。在示例代码中，我们指定了着色器存在的模块`module`和着色器的入口点`entry_point`（也就是会被调用的着色器函数，过会我们会在着色器中看到我们指定的名字），还有对顶点缓冲的描述。你可能会好奇为什么顶点缓冲的描述是一个数组，这是因为一次渲染命令可以同时传入多个顶点缓冲，这被用于一些特殊的渲染中。让我们研究一下单个顶点缓冲的描述：

- _array\_stride_ 描述多少字节表示一个顶点，在我们的情况下自然是每个`Vertex`结构在内存中的大小
- _step\_mode_ 描述这个顶点缓冲内数据在什么情况步进一次并传入顶点着色器处理。这句话听上去莫名其妙，难道顶点缓冲不是一个一个顶点传入的吗？不完全是。目前`wgpu::VertexStepMode`有两个可能值，分别是`VertexStepMode::Vertex`和`VertexStepMode::Instance`。前者便是我们直觉意义上的顶点，后者则是我们称为**实例(_Instance_)**。在之后我们发起渲染请求时，除了指定顶点范围外，还会指定实例范围。在最终渲染时会对每个实例步进一次实例顶点缓冲，然后进行对顶点的处理。在批量绘制静态物品时可以用实例顶点缓冲来储存姿态数据来获得更高的效率。其效果类似下面这段代码：

```rust,no_run
const INSTANCES: usize = 100;
const VERTICES: usize = 3;

struct PerVertexData {
    //每个顶点的数据，在GPU中就是顶点缓冲内每隔一个stride存放的内容
}

struct PerInstanceData {
    //每个实例的数据，在GPU中就是实例顶点缓冲内每隔一个stride存放的内容
}

struct VertexShaderOutput {
    //顶点着色器的输出内容
}

fn vertex_shader(vertex: PerVertexData, instance: PerInstanceData) -> VertexShaderOutput {
    //处理顶点数据并输出
    unimplemented!()
}

let vertex_buffer = [PerVertexData; VERTICES];
let instance_buffer = [PerInstanceData; INSTANCES];

for i in 0..INSTANCES {
    for j in 0..VERTICES {
        let output_vertex = vertex_shader(vertex_buffer[j], instance_buffer[i]);
        // ...
    }
}
```

- _attributes_ 接受一个`wgpu::VertexAttribute`数组切片。其作用是告诉WGPU如何理解每个顶点的数据。在我们的例程中，一个顶点数据在内存中是五个连续存放的单精度浮点，其中前两个描述顶点坐标，后三个描述顶点颜色。在每一个`wgpu::VertexAttribute`中，我们需要指定下面这几个量：
  - _format_ 接受一个`wgpu::VertexFormat`枚举类型。其描述顶点数据中这一部分应当如何对应到顶点着色器理解的类型。例如我们为顶点坐标选择`VertexFormat::Float32x2`，这个类型会直接将两个单精度浮点长度（$2\times4=8$字节）的数据对应到着色器中的二维单精度浮点向量中。除此之外还有很多类型，请读者自行翻看[VertexFormat的文档](https://docs.rs/wgpu/latest/wgpu/enum.VertexFormat.html)
  - _offset_ 表示我们描述的部分应当从单个顶点内存数据的何处算起，这个量和 _format_ 对应的数据大小一起指定了这个属性的内存区域。例如我们有一个顶点数据的字节数组`vertex_bytes: &[u8]`，那么我们的数据便是`vertex_bytes[offset..(offset + format.size())]`
  - _shader\_location_ 是一个`u32`类型，指定我们的数据对应在顶点着色器输入中的“位置”，我们之后将会在顶点着色器中看到其对应的量
  
  而在例程中，我们使用了WGPU提供的一个非常方便的工具宏`wgpu::vertex_attr_array!`，其中会接受一个`loc => format`形式的列表，`loc`对应 _shader\_location_ 而`format`就对应 _format_ 。_offset_ 会由宏内部自动计算，算法是将列表前面每一项的`VertexFormat::size()`累加。因此，只要你的单个顶点数据中的各个部分排布是连续的，就可以使用这个宏！

  在我们的例程中，手写的效果如下：

```rust,no_run
[
    wgpu::VertexAttribute { // 坐标
        format: wgpu::VertexFormat::Float32x2,
        offset: 0 as _,
        shader_location: 0,
    },
    wgpu::VertexAttribute { // 颜色
        format: wgpu::VertexFormat::Float32x3,
        offset: 8 as _,
        shader_location: 1,
    },
]
```

---

### 松散的顶点数据存储

> **注意：** 此部分为之前 **学习过别的图形API** 的读者准备，涉及一些3D渲染的内容。建议初学的读者先[跳过](#内容从这里继续)这一段。

之前学习过OpenGL的读者可能见到过另一种传入顶点数据的方法，即将不同的顶点数据的成分存放在不同的缓冲中，各自组分于内存中是连续的。WGPU也可以做到这一点！为了实现这种做法，我们需要定义多个 _vertex_ 中的`wgpu::VertexBufferLayout`。不妨设我们的一个顶点由下面三个量组成（类型采用`GLSL`）：

- 位置 `layout(location=0) in vec3 position`
- 贴图UV `layout(location=1) in vec2 tex_coord`
- 法向量 `layout(location=2) in vec3 normal`

> `layout(location=x)` 对应于前文的 `wgpu::VertexAttribute.shader_location: x`

那么我们的`wgpu::VertexState.buffers`应当赋予以下值

```rust,no_run
[
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<[f32; 3]>() as _,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3]
    },
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<[f32; 2]>() as _,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![1 => Float32x2]
    },
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<[f32; 3]>() as _,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![2 => Float32x3]
    },
]
```

之后渲染时在对应的槽位绑定数据缓冲即可。

---

### 内容从这里继续

_primitive_ 是对渲染采用的基础形状的描述。看看`wgpu::PrimitiveState`里面都有什么！

- _topology_ 是一个`wgpu::PrimitiveTopology`枚举类型，负责描述我们的顶点数据应当被理解为什么几何图形，目前WGPU仅支持点、线和三角形。读者可以注意到除了点以外的形状都有一个 `*Strip` 变种。简单来说，非`Strip`的类型会将顶点数据划分成不交的一个一个子集，以`PrimitiveTopology::TriangleList`为例，便是三个三个理解为一个三角形，因此顶点缓冲中顶点的数量也必须是3的整数倍。而`Strip`类型则相当于一个滑动窗口，在三角形的情况会尝试从每一个顶点开始往后取三个点（如果可能的话）理解成一个三角形，因而除了顶点数量至少得组成一个三角形以外并没有明显的限制（当然，为了保证面的朝向相同，顶点取出后排列的顺序会有一些调整，详情请参照[文档](https://docs.rs/wgpu/latest/wgpu/enum.PrimitiveTopology.html)）。`Strip`类型的用途在于用尽可能少的顶点数量描述一个连成一片的区域。不过，似乎现实情况下出于泛用性的考虑用得并不是很多。该字段默认为`TriangleList`
- _strip\_index\_format_ 在使用`Strip`类型的图形时索引缓冲的类型。索引缓冲的概念我们将会在下一节学习，目前可以不用在意
- _front\_face_ 图形的定向。默认为逆时针取正向，符合右手螺旋规则。在我们的例程给出的缓冲中，由于顶点是上方$\to$右下$\to$左下的顺序给出的，我们的三角形是被判定为背向，也就是背对屏幕的。该选项在下一个字段 _cull\_mode_ 启用时发挥作用
- _cull\_mode_ 剔除模式。默认为`None`，即不剔除。若开启剔除模式，我们可以选择剔除正向图形或者背向图形。通常我们选择剔除背向图形，因为假定我们选取的正向正确对应了曲面的外法向量，在一个连续不自交的闭合曲面上，一定存在正向图形遮挡了背向图形。然而如果只绘制一个不闭合的图形，那么我们就会有一些方向使得原本看得到的图形被剔除掉。该选项是为了减少绘制看不到的图形而造成的渲染资源浪费设立的，请根据需求选择。
- _unclipped\_depth_ 选择深度是否被裁剪到$\left[0,1\right]$。关于深度的详细知识我们会在3D渲染学习。
- _polygon\_mode_ 多边形模式。这个枚举类型可以决定多边形将怎样被对应到像素上。默认为`PolygonMode::Fill`，也就是填充。根据需求可以选择`PolygonMode::Line`或`PolygonMode::Point`，分别只绘制边线和顶点。注意使用后两者需要启用对应的特性，详情请看[文档](https://docs.rs/wgpu/latest/wgpu/enum.PolygonMode.html)
- _conservative_ 保守光栅化。如果设置为真，则这个多边形最后到屏幕上的投影一定会被其生成的所有像素包含。这可能会导致一些多边形的边缘看着比较粗糙难看。仅当使用`PolygonMode::Fill`时有效，且需要开启对应特性，请看[文档](https://docs.rs/wgpu/latest/wgpu/struct.PrimitiveState.html)

_depth\_stencil_ 是控制我们称为深度模板缓冲的一个功能的选项，让我们在3D渲染的章节再介绍他吧！

_fragment_ 是渲染管线中负责将最终确定要在窗口和屏幕中显示的部分上色的控制，也就是[渲染流程概要](../infra/graphics.md#渲染流程概要)里最后两个节点之间的部分。他可以为`None`，因为有些情况我们完全不需要输出具体的像素。这虽然听上去很奇怪，但是我们将会在本书中用到这个技巧！其中抛去平凡的和 _vertex_ 意义相同的字段以外，我们还剩下一个`target`字段。`target`字段是一个数组切片，因为WGPU支持用一组顶点数据使用不同的片元着色器对多个渲染目标进行输出，从而存储不同的量到最终的输出缓冲里面。目前为止我们只需要输出到唯一一个我们作为帧缓冲的目标里面，所以我们只需要乖乖写入一个`wgpu::ColorTargetState`即可。

那么`wgpu::ColorTargetState`又描述了一些什么呢？

- _format_ 表示将会输出到的目标缓冲的格式。既然我们是要输出到`Surface`维护的帧缓冲，那我们自然要使用和`Surface`相同的格式
- _blend_ 描述如何把这次渲染输出的像素颜色和之前存在于缓冲中的像素颜色混合。通常是用于带透明度图片的渲染，也可以用于一些特效的制作。熟悉图片处理的读者可能知道例如正片叠底之类的混合效果，实际上便是对应该字段描述的内容。对于该字段的细节，我们不妨在接下来的章节介绍
- _write\_mask_ 对这个目标的写入操作的哪些通道有效。是一个`BitSet`，每一位分别代表RGBA通道

很好！我们终于讲完了这个冗长的渲染管线初始化的内容。想必大家读也读累了，喝口水休息一下吧。

最后我们要干的事就很简单了，告诉GPU渲染我们的顶点缓冲。

```rust,no_run
// ...

let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
render_pass.draw(0..3, 0..1);
```

我们告诉WGPU使用刚刚我们创建的渲染管线，然后将槽位0（对应`wgpu::RenderPipelineDescriptor.vertex.buffers`里面的索引，因为我们只有一个元素所以就是0）的顶点缓冲设置为我们创建的顶点缓冲的全部（是的，你可以在这里切出一部分顶点缓冲来用），最后我们指定了渲染顶点的范围（因为我们缓冲内仅有三个顶点来描述我们要画的三角形，需要用到全体顶点，所以是`0..3`，有些需要优化的渲染可以在这里选出顶点缓冲的一部分进行渲染）和实例（我们没有实例缓冲，而且只需要画一个，所以是`0..1`）并创建了渲染命令。

哦，我们好像忘了写着色器了：

```wgsl
// triangle.wgsl

// 暂时不要在意这些@是干什么的
struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

//下面两个函数的函数名就是创建管线的时候指定的 entry_point

//顶点着色器中 @location(x) 里面的 x 对应了 wgpu::VertexAttribute.shader_location
@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) color: vec3<f32>) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(pin: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(pin.color, 1.0);
}
```

为了给本章接下来的章节留一些内容，我们并不会详细解释这个着色器在干什么。不过读者大概可以看出来，我们仅仅是将传进来的顶点位置变成了一个四维的坐标并原样传了出去，并在片元着色器里面直接将顶点传入的颜色当作了输出的颜色。

将着色器放置于适当的位置并编译运行，我们成功在绿的发光的屏幕上看到了白得瞎眼的三角形。这是我们的图形学生涯中令人难忘的第一个三角形！为什么不喝点可乐🥤庆祝一下呢？

> **提示：** 如果出了问题，别忘了和[仓库](https://github.com/switefaster/wgpu-tutor)里面能跑的代码核对一下
