# 一闪一闪亮晶晶——像素与窗口

如有些读者已经察觉的，我们的渲染过程中充斥着各种各样的坐标系。在这一个大章节的后面，我们会花更多章节解释各种坐标系和之间的关系。然而，我们目前为止只涉及了二维的渲染，从而涉及的坐标系大幅减少。在这个章节，我想向读者介绍我们目前主要接触到的几个坐标系。

## 在图形学，我能看到各种各样的坐标

> **易！悟！**
> <p align="right">——理塘王子</p>

### 窗口坐标

我们渲染的结果最终都是呈现到窗口上的。由于窗口的平移之类是系统负责，我们可以不关注屏幕上的绝对坐标，但是我们自然要有办法描述我们渲染的结果在窗口上的相对位置。国际惯例是将窗口的左上角像素取为$\left(0,0\right)$，横向右一个像素为$\hat{x}$坐标单位，纵向下一个像素为$\hat{y}$坐标单位，WGPU中也并不例外。我们的帧缓冲便对应到这个坐标上。

### 视口 _Viewport_

有些情况下，我们希望能在一个窗口上输出不止一个结果。比如左半边渲染一个玩家的视野，右半边渲染另一个玩家的视野之类。尽管这种操作有很多实现方案，视口是其中最直接的一种。视口相当于在窗口中框选出一个区域当作一整个窗口渲染。因此，大部分时候，我们指的“窗口坐标”原则上应当理解成视口坐标（然而也有情况不是）。在我们的教程中，我们的视口永远都是选择整个窗口，所以窗口坐标和视口坐标是一致的。在WGPU中，我们可以通过[`wgpu::RenderPass::set_viewport`](https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html#method.set_viewport)方法来设置视口。前四个参数分别是左上角的位置的x和y窗口坐标，视口的长和宽。后两个参数在3D渲染时才有意义，我们平常将其设置为各自默认值`0.0`和`1.0`即可。不过我们正常不会说“视口坐标”，因为视口内部的坐标从来不出现在我们的计算当中。

### 归一设备坐标 _Normalized Device Coordinates_

归一设备坐标（NDC）是一个三维坐标系$\left[-1,1\right]\times\left[-1,1\right]\times\left[0,1\right]$。其Z轴仅在3D渲染时有意义，故我们暂时忽略Z轴。NDC的坐标会被线性映射到视口坐标中。换句话说，$\left(-1,1\right)$会对应到视口的左上角，而相应的$\left(1,-1\right)$会对应到视口的右下角。我们在上一节的顶点的坐标事实上就是NDC里面的坐标。然而请注意，顶点着色器输出的坐标通常 **不是** NDC，我们将会在3D渲染中更详细地讨论这一事实。

## 顶点着色器和他的坐标朋友们

虽然在二维渲染中我们并不需要处理太多坐标系相关的内容，但是我们已经有了展示顶点着色器作用的绝佳例子了！在通常的二维渲染中，我们需要以像素为单位描述位置，然而这在NDC中是不好做到的。抛开其他的不谈，如果我们采用一个固定的坐标，就会导致我们的图形随着窗口缩放而拉伸。如果我们想要一个固定的大小，就需要我们反复地计算像素宽度对应的NDC宽度。想象一下如果我们有成千上万个（其实通常可以比这更多）图形需要渲染，那我们就需要每一帧计算成千上万的坐标然后将其重新上传至GPU，低效不堪。然而稍微一想，我们就会发现端倪：渲染过程的最后无非是GPU统一把NDC坐标线性变换到视口（在我们的情况下是窗口）中，那我们先让GPU统一进行其逆变换不就可以了吗？顶点着色器就担任了这个对一些顶点统一进行变换的作用。听上去要算的东西还是差不多的，为什么我们更喜欢这样做呢？一是因为这样可以大幅减少CPU和GPU之间相对比较慢的数据交换，因为我们的顶点数据和变换用的数据都可以存在GPU中；二是因为GPU的硬件是对矩阵乘法特化的（不知道为什么和矩阵乘法扯上关系？下一个大章节你就知道了）。这个说法有个地方听上去有些弱智：既然我们本来就想直接输出到窗口坐标，为什么还要有一个NDC呢？在二维渲染的语境下，我没有想到什么为其开脱的理由。但是在三维渲染中，我们将会轻易注意到其重要地位。

> 因为我不想翻转坐标，所以下面像素坐标都是以屏幕左下角为原点，向上为$\hat{y}$，向右为$\hat{x}$

我们还是先实现再说吧！先考虑我们需要什么样一个矩阵进行变换。假设我们窗口的宽度是$w$，高度是$h$（像素），而一个像素的坐标是$\left(x_p, y_p\right)$。这个像素最终应该到达的NDC中的位置记为$\left(x_{NDC},y_{NDC}\right)$，则有：

$$
\begin{cases}
x_{NDC}=\frac{2x_p-w}{w}=\frac{2x_p}{w}-1 \\
y_{NDC}=\frac{2y_p-h}{h}=\frac{2y_p}{h}-1
\end{cases}
$$

是一个缩放加上平移。但平移并不是一个线性变换，这怎么办呢？

> 下面是给有线代基础同学的数学内容，不感兴趣可以选择[跳过](#下课)

---

### 理塘DJ的完美数学教室

在数学中，线性变换配上平移组成的变换被称为 **仿射变换(_Affine Transformation_)**，而仿射变换也是可以用矩阵表示的！一般地说，如果我们想表示一个$\mathbb{R}^n$上的平移$\mathbf{x}\mapsto \mathbf{x}+\mathbf{p}$，那么我们使用一个$n+1$维向量表示$\mathbf{x}$

$$
\mathbf{x}=\begin{bmatrix}x_1\\x_2\\\vdots\\x_n\\1\end{bmatrix}
$$

并用如下矩阵作用于其上

$$
T=\begin{bmatrix}
    1 & 0 & \cdots & 0 & p_1 \\
    0 & 1 & \cdots & 0 & p_2 \\
    \vdots & \vdots & \ddots & \vdots & \vdots \\
    0 & 0 & \cdots & 1 & p_n \\
    0 & 0 & \cdots & 0 & 1
\end{bmatrix}
$$

不难验证

$$
T\mathbf{x}=\begin{bmatrix}x_1+p_1\\x_2+p_2\\\vdots\\x_n+p_n\\1\end{bmatrix}
$$

成功进行了平移。形如$T$的矩阵被称为平移矩阵，我们完全可以把它当作一般的线性变换处理。更一般地说，具有如下形状的分块矩阵：

$$
M=\begin{bmatrix}
    A_{n\times n} & \mathbf{p}_{n\times 1} \\
    0_{1\times n} & 1
\end{bmatrix}
$$

作用于向量

$$
\mathbf{x}_{affine}=\begin{bmatrix}\mathbf{x}_{n\times 1}\\1\end{bmatrix}_{\left(n+1\right)\times 1}
$$

有效果（由分块矩阵乘法易证）

$$\mathbf{x}_{affine} \mapsto \left(A\mathbf{x}+\mathbf{p}\right)_{affine}$$

因此由于在我们的情况下，$A$为矩阵$\begin{bmatrix}\frac{2}{w} & 0\\0 & \frac{2}{h}\end{bmatrix}$，$\mathbf{p}$为向量$\begin{bmatrix}-1\\-1\end{bmatrix}$。因而我们想要的矩阵便是

$$
\begin{bmatrix}
    \frac{2}{w} & 0 & -1 \\
    0 & \frac{2}{h} & -1 \\
    0 & 0 & 1
\end{bmatrix}
$$

但是因为我们需要给三维处理留下一点空间，我们填上$\hat{z}$的单位映射得到

$$
\begin{bmatrix}
    \frac{2}{w} & 0 & 0 & -1 \\
    0 & \frac{2}{h} & 0 & -1 \\
    0 & 0 & 1 & 0 \\
    0 & 0 & 0 & 1
\end{bmatrix}
$$

这便是我们最终需要使用的矩阵了！

---

### 下课

给跳过的同学贴上省流版本：我们只需要将这个矩阵

$$
\begin{bmatrix}
    \frac{2}{w} & 0 & 0 & -1 \\
    0 & \frac{2}{h} & 0 & -1 \\
    0 & 0 & 1 & 0 \\
    0 & 0 & 0 & 1
\end{bmatrix}
$$

乘到

$$
\mathbf{x}=\begin{bmatrix}x_p\\y_p\\z\\1\end{bmatrix}
$$

上，就能在前两个分量中得到我们想要的结果。其中$z$的值随意，但是由于NDC的$\hat{z}$取值范围不能超过$\left[0,1\right]$，这里的$z$也不可以超过$\left[0,1\right]$。

## 实操时间

为了方便在程序中操作矩阵，我们引入一个新的依赖

```toml
# Cargo.toml

[dependencies]
# ...
cgmath = "0.18"
```

我们先更新一下我们的顶点数据：

```rust,no_run
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
```

记住，经过我们的操作后，我们现在采取的是窗口的像素坐标：原点在窗口左下角，单位为像素。因此，这个三角形是一个顶点位于窗口左下角，底边贴着窗口底边，底为100像素，高为100像素的白色三角形。

然后在初始化代码中的某一段生成矩阵并把它写入缓冲

```rust,no_run
let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: None,
    contents: bytemuck::cast_slice(&triangle),
    usage: wgpu::BufferUsages::VERTEX,
});

// 从这里继续

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

// 虽然我们最终还是把矩阵转成了二维数组，但是反正我们迟早需要在CPU里面计算矩阵乘法，不如早点引入cgmath
let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: None,
    contents: bytemuck::cast_slice(&<cgmath::Matrix4<f32> as Into<[[f32; 4]; 4]>>::into(
        pixel_matrix,
    )),
    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
});
```

> `cgmath`的内部存储结构和输入时的顺序都是 **列优先(_Column Major_)** 的，和WGPU一致。换句话说，我们输入的数字四个四个分为一组分别为第一列到第四列。如果你把它每四个换一行看，那看到的就是我们需要的矩阵的转置。这个比较误导人，请务必注意！

注意到我们的缓冲的用途是`wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST`。前者告诉WGPU我们的缓冲将会用于向着色器内传递至少在一次渲染调用内保持不变的数据，后者则允许我们在必要时更新缓存的内容：例如窗口大小变化时。

接下来，我们需要想办法将数据传递进着色器中。还记得上一节基本留空的管线布局(_Pipeline Layout_)吗？我们提到过向着色器传入数据和它有关。准确的说，和其中的绑定组(_Bind Group_)有关。

绑定组存在的目的是向着色器传递在一次或者一次以上绘制请求之间不变的数据（与顶点数据不同）。为了顺利传递一个绑定组，我们需要如下操作：

- 初始化时
  - 创建绑定组布局(_Bind Group Layout_)
  - 创建拥有绑定组布局引用的管线布局
  - 创建拥有对应绑定组布局的管线
  - 创建绑定组
- 渲染时
  - 使用对应的管线
  - 将绑定组设置到对应槽位
  - 发起渲染请求

那么我们便开始吧。我们先创建好绑定组布局和绑定组

```rust,no_run
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
```

如你所见，我们的绑定组布局和绑定组都拥有多个条目(_entry_)，而且他们得是对应的。这就是说，一个绑定组可以同时传入多种不同的数据。因此，我们可以用绑定组将一些永远同时出现的数据放在一起传入（比如渲染一个物体时，我们可以将其姿态和材质放在同一个绑定组传入）。每个不管是绑定组布局还是绑定组， _entry_ 都拥有一个`binding`字段，其指示着色器之后将索引该绑定组里面的数据，我们之后将会看到这一点。

太棒了，现在我们可以更新我们的管线布局来告诉WGPU我们想要在使用这个管线时传入这一种绑定组了：

```rust,no_run
let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: None,
    bind_group_layouts: &[&matrix_bind_group_layout], // 改动了这里
    push_constant_ranges: &[],
});
```

### 数组顺序

尽管我们这里并不需要传入多个绑定组布局，但是我仍需要提醒的是，在传入多个绑定组布局时，`bind_group_layouts`中元素的顺序必须和绑定组的槽位编号一致。之后我们需要用传入多个绑定组时会再强调这一点。

也别忘了更新我们的着色器：

```rust,no_run
let shader_module =
    device.create_shader_module(wgpu::include_wgsl!("triangle/triangle-pixel.wgsl"));
```

渲染管线本身的创造则并没有什么需要改动的地方。

最后在渲染之前记得设置一下绑定组

```rust,no_run
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
render_pass.set_bind_group(0, &matrix_bind_group, &[]); // 在这里绑定
render_pass.draw(0..3, 0..1);
```

设置绑定组的第一个参数是绑定组的槽位，和[这里所说](#数组顺序)是对应的，第二个参数是对绑定组的引用（注意不是布局，只有绑定组本身才负责掌管具体的数据），第三个参数则是动态偏移需要的，我们用不到。

原则上只要在你更新了着色器的内容（在下文）以后，我们的渲染其实已经符合我们的预期了：我们会看到一个贴在屏幕左下角的，底边和高都是100像素的三角形。然而一旦窗口的大小发生变化，我们的矩阵也是需要更新的：因为它的值依赖于窗口的长和宽。为了无论如何缩放窗口，我们都能有正确的三角形大小，我们需要的合适的时候更新矩阵缓冲的内容。而这个更新，正应该发生在窗口大小缩放时：

```rust,no_run
winit::event::WindowEvent::Resized(new_size) => {
    if new_size.width > 0 && new_size.height > 0 {
        surface_config.width = new_size.width;
        surface_config.height = new_size.height;
        surface.configure(&device, &surface_config);

        // 新增内容
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
```

几乎就是我们创造矩阵缓冲时代码的翻版。`Queue::write_buffer`会在当前队列的最开始，也就是下一次渲染开始之前，插入一条上传缓冲的指令。它的第一个参数是需要写入的缓冲的引用，第二条是写入位置从缓冲开始位置的偏移值（这个参数是为了方便局部更新），第三个参数是需要写入的字节数据。

最后我们来看一下着色器需要有什么变化：

```wgsl
// triangle/triangle-pixel.wgsl

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@group(0)
@binding(0)
var<uniform> u_projection: mat4x4<f32>;

@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) color: vec3<f32>) -> VertexOut {
    var out: VertexOut;
    out.position = u_projection * vec4<f32>(position, 0.0, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(pin: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(pin.color, 1.0);
}
```

我们仍然暂时忽略在上一节便出现过的各种`@location`和`@builtin`，因为这是下一节的内容，而把注意力集中在多出来的部分。首先我们可以注意到的改动是

```wgsl
@group(0)
@binding(0)
var<uniform> u_projection: mat4x4<f32>;
```

这一部分会负责接收从绑定组传来的数据。`@group(0)`表示我们的绑定组会从槽位`0`被传入，与[这里所说](#数组顺序)对应，也和`render_pass.set_bind_group`的参数对应。而`@binding(0)`则表示我们接收到这个变量的数据是这个绑定组的第`0`个条目。由于我们从槽位`0`传入的绑定组的布局是入口点`0`为一个缓冲，着色器会尝试将这个缓冲理解为我们下面指定的类型。接下来的`var<uniform>`则告诉着色器接下来定义的变量是从绑定组接收的数据，这和GLSL中的`uniform`意义一致。然后我们定义其名称为`u_projection`，类型为`mat4x4<f32>`，也就是内容是`f32`的$4\times 4$矩阵。变量名字是可以随意起的。在接下来几个章节，我们将会面对条目越来越多的绑定组，也会同时用到多个绑定组。

总的来说，这一段就是在表示：我要用第`0`绑定组的第`0`个条目的数据，而且要把那一段内存理解成$4\times 4$的单精度浮点矩阵。而经过这一番操作，我们在程序中传入的矩阵就可以在着色器中被使用了，而我们的用法也很简单，就是像前文说的那样，把它乘到顶点坐标上而已。

```wgsl
out.position = u_projection * vec4<f32>(position, 0.0, 1.0);
```

真不错！

现在，你窗口的左下角有了一个大小不变而且死皮赖脸贴着窗口的小三角形了。你肯定早已注意到我们的顶点数据中有着描述颜色的字段，而你肯定充满了好奇：三个顶点的颜色，怎么能决定整个三角形每个像素的颜色？同时，我们的着色器中还充斥着各种神奇的标记，我们还没对它们做出任何解释。不用怕，一切都会在下一节真相大白！**拖更道，堂堂连载！**
